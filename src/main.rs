use std::io::{Error, ErrorKind};

use futures::future::{ok, Future};
use futures::stream::Stream;
use tokio::io::{read_exact, write_all};
use tokio::net::{TcpListener, TcpStream};
use futures::sink::Sink;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::io::{AsyncRead, ReadHalf, WriteHalf};
use tokio_serde_json::{ReadJson, WriteJson};

use errors::RpcError;
use messages::{Response, Request};

mod messages;
mod errors;

pub type Result<T> = std::result::Result<T, RpcError>;

fn serve(tcp: TcpStream) -> impl Future<Item = (), Error = RpcError> {
    let (read_half, write_half) = tcp.split();
    let read_json = ReadJson::new(FramedRead::new(read_half, LengthDelimitedCodec::new()));
    let resp_stream = read_json
        .map_err(RpcError::from)
        .and_then(
            move |req| -> Box<dyn Future<Item = Response, Error = RpcError> + Send> {
                match req {
                    Request::Get(num) => {
                        println!("get {}", num);
                        Box::new(ok(Response::Urls(Vec::new())))
                    },
                    Request::GetAnon(num) => {
                        println!("get anon {}", num);
                        Box::new(ok(Response::Urls(Vec::new())))
                    },
                    Request::Set(values) => {
                        println!("set {:?}", values);
                        Box::new(ok(Response::Set))
                    }
                    Request::Join => {
                        println!("join");
                        Box::new(ok(Response::Join))
                    }
                }
            },
        )
        .then(|resp| -> Result<Response> {
            match resp {
                Ok(resp) => Ok(resp),
                Err(e) => Ok(Response::Err(format!("{}", e))),
            }
        });
    let write_json = WriteJson::new(FramedWrite::new(write_half, LengthDelimitedCodec::new()));
    write_json
        .sink_map_err(RpcError::from)
        .send_all(resp_stream)
        .map(|_| ())
}

fn main() {
    let addr = "127.0.0.1:10001".parse().unwrap();

    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    let server = listener
        .incoming()
        .map_err(|e| eprintln!("failed to accept stream; error = {:?}", e))
        .for_each(move |socket| {
            println!("new socket!  {}", socket.peer_addr().unwrap());
            // The initial greeting from the client
            //      field 1: version, 1 byte (0x01 for this version)
            //      field 2: number of authentication methods supported, 1 byte
            
            serve(socket).map_err(|e| eprintln!("failed to accept stream; error = {:?}", e))
        });

    tokio::run(server);
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