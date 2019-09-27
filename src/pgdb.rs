use std::thread;

use crossbeam::channel::{select, Receiver};
use dotenv::var;
use postgres::{Connection, TlsMode};

use crate::errors::RpcError;
use crate::proxy::Proxy;
use crate::types::RProxy;

pub struct PgDb {
    pub db: Connection,
    pub workers: RProxy,
}

impl PgDb {
    fn new(db: Connection, workers: Receiver<Proxy>) -> Self {
        PgDb { db, workers }
    }

    pub fn start(workers: Receiver<Proxy>) {
        let db = get_connection().expect("error in connecting to pg db");
        let pg_db = PgDb::new(db, workers);
        thread::spawn(move || pg_db.run());
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.workers) -> msg => {
                    if let Ok(proxy) = msg {
                        let _ = insert_or_update(&self.db, proxy);
                    }
                }
            }
        }
    }
}

fn get_connection() -> Result<Connection, RpcError> {
    let params = var("PG").expect("No found variable PG like postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]] in environment");
    Ok(Connection::connect(params, TlsMode::None)?)
}

pub fn insert_or_update(conn: &Connection, proxy: Proxy) -> Result<u64, RpcError> {
    Ok(conn.execute(
        "INSERT INTO
            proxies (work, anon, checks, hostname, host, port, scheme, create_at, update_at, response)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT
            (hostname)
        DO UPDATE SET
            (work, anon, checks, update_at, response) =
            ($1, $2, $3 + 1, $9, $10)
        ",
        &[
            &proxy.work,
            &proxy.anon,
            &proxy.checks,
            &proxy.hostname,
            &proxy.host,
            &proxy.port,
            &proxy.scheme,
            &proxy.create_at,
            &proxy.update_at,
            &proxy.response,
        ])?)
}
