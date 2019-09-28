use crossbeam::channel::{select, unbounded};
// use sled::Db;
use std::thread;

use crate::errors::RpcError;
use crate::types::{RVecStr, SStr, RcvWorkExt, SndWorkExt, WorkExt};
use crate::worker::Worker;

pub struct Manager {
    w_sender: SndWorkExt,
    w_receiver: RcvWorkExt,
    // server: RVecStr,
    // workers: SStr,
    // db: Db,
}

impl Manager {
    fn new(
        w_sender: SndWorkExt,
        w_receiver: RcvWorkExt,
        // server: RVecStr,
        // workers: SStr,
        // db_name: String,
    ) -> Result<Manager, RpcError> {
        // let db = Db::open(db_name).map_err(|e| e.to_string())?;
        Ok(Manager {
            w_sender,
            w_receiver,
            // db,
        })
    }

    pub fn start(
        server: RVecStr,
        workers: SStr,
        // db_name: String,
    ) -> Result<(), RpcError> {
        // let manager = Manager::new(server, workers, db_name)?;
        let (worker_r, worker_s) = unbounded();
        Worker::start(worker_s, worker_r);
        let manager = Manager::new(worker_r, worker_s)?;
        thread::spawn(move || manager.run());
        Ok(())
    }

    fn run(&self) {
        loop {
            select! {
                // recv(self.worker_r) -> msg => {
                //     if let Ok(url_list) = msg {
                //         for url in url_list {
                //             // if self.db.insert(url.clone(), b"") == Ok(None) {
                //                 if url.contains("://") {
                //                     let _ = self.workers.send(url);
                //                 } else {
                //                     let _ = self.workers.send(format!("http://{}", url));
                //                     let _ = self.workers.send(format!("socks5://{}", url));
                //                 }
                //             // }
                //         }
                //     }
                // }
                recv(self.w_receiver) -> msg => {
                    if let Ok(WorkExt::Proxy(proxy)) = msg {
                        let _ = self.workers.send(url);
                    }
                }
            }
        }
    }
}
