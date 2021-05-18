use crossbeam::channel::select;
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

use crate::errors::RssError;
use crate::messages::{PgExt, PgGetter, RcvPgExt, SndPgExt};
use crate::proxy::Proxy;

pub struct PgDb {
    pub db: Pool,
    pub sender: SndPgExt,
    pub receiver: RcvPgExt,
}

impl PgDb {
    fn new(db: Pool, sender: SndPgExt, receiver: RcvPgExt) -> Self {
        PgDb {
            db,
            receiver,
            sender,
        }
    }

    pub async fn start(sender: SndPgExt, receiver: RcvPgExt) {
        let db = get_pool();
        let pg_db = PgDb::new(db, sender, receiver);
        tokio::spawn(async move { pg_db.run().await });
    }

    async fn run(&self) {
        loop {
            select! {
                recv(self.receiver) -> msg => match msg {
                    Ok(PgExt::Proxy(proxy)) => {
                        let _ = self.insert_or_update(proxy);
                    },
                    Ok(PgExt::Get(getter)) => {
                        let urls = self.get_list(getter);
                        let _ = self.sender.send(PgExt::Urls(urls.await));
                    },
                    _ => (),
                }
            }
        }
    }

    async fn insert_or_update(&self, proxy: Proxy) -> Result<u64, RssError> {
        Ok(self.db.get().await?.execute(
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
            ]).await?)
    }

    async fn get_list(&self, msg: PgGetter) -> Vec<String> {
        let mut proxies = Vec::new();
        if let Ok(client) = self.db.get().await {
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

            if let Ok(rows) = client.query(query.as_str(), &[&msg.work, &msg.limit]).await {
                for row in rows {
                    let hostname: String = row.get(0);
                    proxies.push(hostname);
                }
            }
        }
        proxies
    }
}

fn get_config() -> Config {
    let pg_url = dotenv::var("PG_RSS").expect("No found variable PG_RS like postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]] in environment");
    pg_url.parse().expect("no parge config from PG_RSS")
}

pub fn get_pool() -> Pool {
    let manager = Manager::new(get_config(), NoTls);
    Pool::new(manager, 16)
}
