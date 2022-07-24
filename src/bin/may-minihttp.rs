extern crate may_minihttp;
use may_minihttp::{HttpService, HttpServiceFactory, Request, Response};
use std::io;

/// `HelloWorld` is the *service* that we're going to be implementing to service
/// the HTTP requests we receive.
///
#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, _req: Request, rsp: &mut Response) -> io::Result<()> {
        rsp.body("Hello, world!");
        Ok(())
    }
}

struct HelloWorldFac;

impl HttpServiceFactory for HelloWorldFac {
    type Service = HelloWorld;

    fn new_service(&self) -> Self::Service {
        HelloWorld
    }
}

fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let server = HelloWorldFac
        .start(format!("127.0.0.1:{}", port_number))
        .unwrap();
    server.wait();
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
