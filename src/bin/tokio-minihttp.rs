extern crate tokio_minihttp;
use std::io;

use futures::future;
use tokio_minihttp::{Http, Request, Response};
use tokio_proto::TcpServer;
use tokio_service::Service;

/// `HelloWorld` is the *service* that we're going to be implementing to service
/// the HTTP requests we receive.
///
/// The tokio-minihttp crate, and much of Tokio itself, are centered around the
/// concept of a service for interoperability between crates. Our service here
/// carries no data with it.
///
/// Note that a new instance of `HelloWorld` is created for each TCP connection
/// we service, created below in the closure passed to `serve`.
struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.body("Hello, world!");
        future::ok(resp)
    }
}

fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let addr = format!("127.0.0.1:{}", port_number).parse().unwrap();
    TcpServer::new(Http, addr).serve(|| Ok(HelloWorld));
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
