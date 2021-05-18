use std::thread;

use crossbeam::channel::{select, unbounded};
use dotenv::var;
use sled::Db;

use crate::errors::RssError;
use crate::messages::{
    PgExt, RcvPgExt, RcvSrvExt, RcvWorkExt, SndPgExt, SndSrvExt, SndWorkExt, WorkExt,
};
use crate::pgdb::PgDb;
use crate::worker::Worker;

pub struct Manager {
    srv_sender: SndSrvExt,
    srv_receiver: RcvSrvExt,
    w_sender: SndWorkExt,
    w_receiver: RcvWorkExt,
    pg_sender: SndPgExt,
    pg_receiver: RcvPgExt,
    sled: Db,
}

impl Manager {
    fn new(
        srv_sender: SndSrvExt,
        srv_receiver: RcvSrvExt,
        w_sender: SndWorkExt,
        w_receiver: RcvWorkExt,
        pg_sender: SndPgExt,
        pg_receiver: RcvPgExt,
    ) -> Result<Manager, RssError> {
        let sled_db_name = var("SLED").expect("No found variable sled like SLED in environment");
        let sled = sled::open(sled_db_name)?;
        Ok(Manager {
            srv_sender,
            srv_receiver,
            w_sender,
            w_receiver,
            pg_sender,
            pg_receiver,
            sled,
        })
    }

    pub async fn start(srv_sender: SndSrvExt, srv_receiver: RcvSrvExt) -> Result<(), RssError> {
        let (worker_s, worker_r) = unbounded();
        Worker::start(worker_s.clone(), worker_r.clone()).await;
        let (pgdb_s, pgdb_r) = unbounded();
        PgDb::start(pgdb_s.clone(), pgdb_r.clone()).await;
        let manager = Manager::new(srv_sender, srv_receiver, worker_s, worker_r, pgdb_s, pgdb_r)?;
        thread::spawn(move || manager.run());
        Ok(())
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.srv_receiver) -> msg => match msg {
                    Ok(PgExt::Urls(url_list)) => {
                        for url in url_list {
                            if self.sled.insert(url.clone(), b"") == Ok(None) {
                                if url.contains("://") {
                                    let _ = self.w_sender.send(WorkExt::Url(url));
                                } else {
                                    let _ = self.w_sender.send(WorkExt::Url(format!("http://{}", url)));
                                    let _ = self.w_sender.send(WorkExt::Url(format!("socks5://{}", url)));
                                }
                            }
                        }
                    },
                    _ => (),
                },
                recv(self.w_receiver) -> msg => {
                    if let Ok(WorkExt::Proxy(proxy)) = msg {
                        let _ = self.pg_sender.send(PgExt::Proxy(proxy));
                    }
                },
                recv(self.pg_receiver) -> msg => match msg {
                    Ok(PgExt::Urls(url_list)) => {
                        let _ = self.srv_sender.send(PgExt::Urls(url_list));
                    },
                    _ => (),
                },
            }
        }
    }
}
