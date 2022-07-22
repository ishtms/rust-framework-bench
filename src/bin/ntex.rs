use std::io;

use ntex::http::{HttpService, Response};
use ntex::{server::Server, util::Ready};

#[ntex::main]
async fn main() -> io::Result<()> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    Server::build()
        .bind("hello-world", format!("127.0.0.1:{}", port_number), |_| {
            HttpService::build().finish(|_req| {
                let mut res = Response::Ok();
                Ready::Ok::<_, io::Error>(res.body("Hello world!"))
            })
        })?
        .run()
        .await
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
