use crossbeam::channel::{select};
use dotenv::var;
use std::thread;

use crate::proxy::{check_proxy};
use crate::utils::my_ip;
use crate::types::{RStr, SProxy};

pub struct Worker {
    pub id: usize,
    pub ip: String,
    pub target: String,
    pub server: RStr,
    pub db_saver: SProxy,
}

impl Worker {
    fn new(
        id: usize,
        ip: String,
        target: String,
        server: RStr,
        db_saver: SProxy,
    ) -> Self {
        Worker {
            id,
            ip,
            target,
            server,
            db_saver,
        }
    }

    pub fn start(worker_r: RStr, worker_s: SProxy) {
        let target = var("TARGET")
            .expect("No found variable target like http://targethost:433/path in environment");
        let num_workers = var("WORKERS")
            .expect("No found variable workers like 4 in environment")
            .parse::<usize>()
            .expect("wrong variable workers in environment");
        let ip = my_ip().expect("error get ip");
        for i in 0..num_workers {
            let r = worker_r.clone();
            let s = worker_s.clone();
            let ip = ip.clone();
            let target = target.clone();
            thread::spawn(move || {
                let worker = Worker::new(i, ip, target, r, s);
                worker.run();
            });
        }
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.server) -> msg => {
                    if let Ok(proxy_url) = msg {
                        if let Ok(proxy) = check_proxy(&proxy_url, &self.target, &self.ip) {
                            let _ = self.db_saver.send(proxy);
                        }
                    }
                }
            }
        }
    }
}
