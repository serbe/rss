use crossbeam::channel::unbounded;
use dotenv::dotenv;
use tokio::runtime::Runtime;

use errors::RssError;
use manager::Manager;
use server::s;

mod errors;
mod manager;
mod messages;
mod pgdb;
mod proxy;
mod server;
mod utils;
mod worker;

// pub type Result<T> = std::result::Result<T, RssError>;

async fn run() -> Result<(), RssError> {
    dotenv().ok();

    // let (server_s, manager_r) = unbounded();
    // let (manager_s, worker_r) = unbounded();
    // let (worker_s, saver_r) = unbounded();
    let (server_s, server_r) = unbounded();
    // Manager::start(manager_r, manager_s, cfg.sled)?;
    Manager::start(server_s.clone(), server_r.clone()).await?;

    // Worker::start(worker_r, worker_s);

    let server = s(server_s, server_r).await;

    server
}

// let handshake = read_exact(socket, [0u8; 2])
//                 .and_then(|(socket, buf)| {
//                     println!("get request from client {:?}", buf);
//                     match buf {
//                         [1u8, 0u8] => {
//                             println!("connect");
//                             Ok(socket)
//                         }
//                         _ => Err(Error::from(ErrorKind::InvalidData)),
//                     }
//                 })
//                 .and_then(|socket| write_all(socket, [1u8, 0u8]));

//             let finisher = handshake
//                 .and_then(|_socket| {
//                     println!("finish");
//                     Ok(())
//                 })
//                 .map_err(|e| eprintln!("error = {:?}", e));

//             tokio::spawn(finisher);

fn main() {
    dotenv::dotenv().ok();

    let rt = Runtime::new().unwrap();

    rt.block_on(async { run().await.unwrap() });
}
