use dotenv::var;
// use futures::future::{ok, Future};
// use futures::sink::Sink;
// use futures::stream::Stream;
// use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};

use crate::errors::RssError;
use crate::messages::{RcvSrvExt, Request, Response, SndSrvExt};

async fn serve(
    socket: TcpStream,
    srv_sender: SndSrvExt,
    srv_receiver: RcvSrvExt,
) -> Result<(), RssError> {
    let mut lines = Framed::new(socket, LinesCodec::new());
    while let Some(result) = lines.next().await {
        match result {
            Ok(line) => {
                let response = handle_request(&line, &db);

                let response = response.serialize();

                if let Err(e) = lines.send(response.as_str()).await {
                    println!("error on sending response; error = {:?}", e);
                }
            }
            Err(e) => {
                println!("error on decoding from socket; error = {:?}", e);
            }
        }
    }
    Ok(())
}

pub async fn s(srv_sender: SndSrvExt, srv_receiver: RcvSrvExt) -> Result<(), RssError> {
    let server_addr =
        var("SERVER").expect("No found variable SERVER like 0.0.0.0:8080 in environment");

    let listener = TcpListener::bind(&server_addr).await?;

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new socket!  {}", addr);
                serve(socket, srv_sender.clone(), srv_receiver.clone()).await;
            }
            Err(err) => {
                println!("error accepting socket; error = {:?}", err);
            }
        }
    }
}
