use crossbeam::channel::{select, unbounded};
// use sled::Db;
use std::thread;

use crate::errors::RpcError;
use crate::pgdb::PgDb;
use crate::types::{
    PgExt, PgGetter, RVecStr, RcvPgExt, RcvSrvExt, RcvWorkExt, SndPgExt, SndSrvExt, SndWorkExt,
    WorkExt,
};
use crate::worker::Worker;

pub struct Manager {
    srv_sender: SndSrvExt,
    srv_receiver: RcvSrvExt,
    w_sender: SndWorkExt,
    w_receiver: RcvWorkExt,
    pg_sender: SndPgExt,
    pg_receiver: RcvPgExt,
    // server: RVecStr,
    // workers: SStr,
    // db: Db,
}

impl Manager {
    fn new(
        srv_sender: SndSrvExt,
        srv_receiver: RcvSrvExt,
        w_sender: SndWorkExt,
        w_receiver: RcvWorkExt,
        pg_sender: SndPgExt,
        pg_receiver: RcvPgExt,
        // server: RVecStr,
        // workers: SStr,
        // db_name: String,
    ) -> Result<Manager, RpcError> {
        // let db = Db::open(db_name).map_err(|e| e.to_string())?;
        Ok(Manager {
            srv_sender,
            srv_receiver,
            w_sender,
            w_receiver,
            pg_sender,
            pg_receiver,
            // db,
        })
    }

    pub fn start(
        srv_sender: SndSrvExt,
        srv_receiver: RcvSrvExt,
        // db_name: String,
    ) -> Result<(), RpcError> {
        let (worker_s, worker_r) = unbounded();
        Worker::start(worker_s.clone(), worker_r.clone());
        let (pgdb_s, pgdb_r) = unbounded();
        PgDb::start(pgdb_s.clone(), pgdb_r.clone());
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
                            // if self.db.insert(url.clone(), b"") == Ok(None) {
                                if url.contains("://") {
                                    let _ = self.w_sender.send(WorkExt::Url(url));
                                } else {
                                    let _ = self.w_sender.send(WorkExt::Url(format!("http://{}", url)));
                                    let _ = self.w_sender.send(WorkExt::Url(format!("socks5://{}", url)));
                                }
                            // }
                        }
                    },
                    _ => (),
                },
                recv(self.w_receiver) -> msg => {
                    if let Ok(WorkExt::Proxy(proxy)) = msg {
                        let _ = self.pg_sender.send(PgExt::Proxy(proxy));
                    }
                }
            }
        }
    }
}
