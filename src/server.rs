use dotenv::var;
use futures::future::{ok, Future};
use futures::sink::Sink;
use futures::stream::Stream;
use tokio::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio::io::AsyncRead;
use tokio::net::{TcpListener, TcpStream};
use tokio_serde_json::{ReadJson, WriteJson};

use crate::errors::RpcError;
use crate::messages::{Request, Response};

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
                    }
                    Request::GetAnon(num) => {
                        println!("get anon {}", num);
                        Box::new(ok(Response::Urls(Vec::new())))
                    }
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
        .then(|resp| -> Result<Response, RpcError> {
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

pub fn s() -> impl Future<Item = (), Error = ()> {
    let server_addr =
        var("SERVER").expect("No found variable SERVER like 0.0.0.0:8080 in environment");
    let addr = server_addr.parse().expect("error parse server address");

    let listener = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    listener
        .incoming()
        .map_err(|e| eprintln!("failed to accept stream; error = {:?}", e))
        .for_each(move |socket| {
            println!("new socket!  {}", socket.peer_addr().unwrap());
            // The initial greeting from the client
            //      field 1: version, 1 byte (0x01 for this version)
            //      field 2: number of authentication methods supported, 1 byte

            serve(socket).map_err(|e| eprintln!("failed to accept stream; error = {:?}", e))
        })
}
