extern crate may_minihttp;
use may_minihttp::{HttpServer, HttpService, Request, Response};
use std::io;

/// `HelloWorld` is the *service* that we're going to be implementing to service
/// the HTTP requests we receive.
///
#[derive(Clone)]
struct HelloWorld;

impl HttpService for HelloWorld {
    fn call(&mut self, _req: Request, res: &mut Response) -> io::Result<()> {
        res.body("Hello, world!");
        Ok(())
    }
}

fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let server = HttpServer(HelloWorld)
        .start(format!("127.0.0.1:{}", port_number))
        .unwrap();
    server.wait();
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
