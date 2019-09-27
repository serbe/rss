use crossbeam::channel::select;
// use sled::Db;
use std::thread;

use crate::errors::RpcError;
use crate::types::{RVecStr, SStr};

pub struct Manager {
    server: RVecStr,
    workers: SStr,
    // db: Db,
}

impl Manager {
    fn new(
        server: RVecStr,
        workers: SStr,
        // db_name: String,
    ) -> Result<Manager, RpcError> {
        // let db = Db::open(db_name).map_err(|e| e.to_string())?;
        Ok(Manager {
            server,
            workers,
            // db,
        })
    }

    pub fn start(
        server: RVecStr,
        workers: SStr,
        // db_name: String,
    ) -> Result<(), RpcError> {
        // let manager = Manager::new(server, workers, db_name)?;
        let manager = Manager::new(server, workers)?;
        thread::spawn(move || manager.run());
        Ok(())
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.server) -> msg => {
                    if let Ok(url_list) = msg {
                        for url in url_list {
                            // if self.db.insert(url.clone(), b"") == Ok(None) {
                                if url.contains("://") {
                                    let _ = self.workers.send(url);
                                } else {
                                    let _ = self.workers.send(format!("http://{}", url));
                                    let _ = self.workers.send(format!("socks5://{}", url));
                                }
                            // }
                        }
                    }
                }
            }
        }
    }
}
