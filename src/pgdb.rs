use std::thread;

use crossbeam::channel::select;
use dotenv::var;
use postgres::{Connection, TlsMode};

use crate::errors::RpcError;
use crate::messages::{PgExt, PgGetter, RcvPgExt, SndPgExt};
use crate::proxy::Proxy;

pub struct PgDb {
    pub db: Connection,
    pub sender: SndPgExt,
    pub receiver: RcvPgExt,
}

impl PgDb {
    fn new(db: Connection, sender: SndPgExt, receiver: RcvPgExt) -> Self {
        PgDb {
            db,
            receiver,
            sender,
        }
    }

    pub fn start(sender: SndPgExt, receiver: RcvPgExt) {
        let db = get_connection().expect("error in connecting to pg db");
        let pg_db = PgDb::new(db, sender, receiver);
        thread::spawn(move || pg_db.run());
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.receiver) -> msg => match msg {
                    Ok(PgExt::Proxy(proxy)) => {
                        let _ = self.insert_or_update(proxy);
                    },
                    Ok(PgExt::Get(getter)) => {
                        let urls = self.get_list(getter);
                        let _ = self.sender.send(PgExt::Urls(urls));
                    },
                    _ => (),
                }
            }
        }
    }

    fn insert_or_update(&self, proxy: Proxy) -> Result<u64, RpcError> {
        Ok(self.db.execute(
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

    fn get_list(&self, msg: PgGetter) -> Vec<String> {
        let mut proxies = Vec::new();
        let anon = match msg.anon {
            Some(value) => format!("AND anon = {}", value),
            None => String::new(),
        };
        let hours = match msg.hours {
            Some(value) => format!("AND update_at < (NOW() - interval '{} hour')", value),
            None => String::new(),
        };
        let query = format!(
            "SELECT
                hostname
            FROM
                proxies
            WHERE
                work = $1 {} {} AND random() < 0.01
            LIMIT $2",
            anon, hours
        );
        if let Ok(rows) = &self.db.query(&query, &[&msg.work, &msg.limit]) {
            for row in rows {
                let hostname: String = row.get(0);
                proxies.push(hostname);
            }
        }
        proxies
    }
}

fn get_connection() -> Result<Connection, RpcError> {
    let params = var("PG").expect("No found variable PG like postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]] in environment");
    Ok(Connection::connect(params, TlsMode::None)?)
}

