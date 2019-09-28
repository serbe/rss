use crossbeam::channel::select;
use dotenv::var;
use std::thread;

use crate::proxy::check_proxy;
use crate::types::{RcvWorkExt, SndWorkExt, WorkExt};
use crate::utils::my_ip;

pub struct Worker {
    pub id: usize,
    pub ip: String,
    pub target: String,
    pub receiver: RcvWorkExt,
    pub sender: SndWorkExt,
}

impl Worker {
    fn new(
        id: usize,
        ip: String,
        target: String,
        sender: SndWorkExt,
        receiver: RcvWorkExt,
    ) -> Self {
        Worker {
            id,
            ip,
            target,
            sender,
            receiver,
        }
    }

    pub fn start(w_sender: SndWorkExt, w_receiver: RcvWorkExt) {
        let target = var("TARGET")
            .expect("No found variable target like http://targethost:433/path in environment");
        let num_workers = var("WORKERS")
            .expect("No found variable workers like 4 in environment")
            .parse::<usize>()
            .expect("wrong variable workers in environment");
        let ip = my_ip().expect("error get ip");
        for i in 0..num_workers {
            let receiver = w_receiver.clone();
            let sender = w_sender.clone();
            let ip = ip.clone();
            let target = target.clone();
            thread::spawn(move || {
                let worker = Worker::new(i, ip, target, sender, receiver);
                worker.run();
            });
        }
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.receiver) -> msg => {
                    if let Ok(WorkExt::Url(proxy_url)) = msg {
                        if let Ok(proxy) = check_proxy(&proxy_url, &self.target, &self.ip) {
                            let _ = self.sender.send(WorkExt::Proxy(proxy));
                        }
                    }
                }
            }
        }
    }
}
