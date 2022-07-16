use std::io;

use ntex::http::{HttpService, Response};
use ntex::{server::Server, util::Ready};

#[ntex::main]
async fn main() -> io::Result<()> {
    Server::build()
        .bind("hello-world", "127.0.0.1:3003", |_| {
            HttpService::build().finish(|_req| {
                let mut res = Response::Ok();
                Ready::Ok::<_, io::Error>(res.body("Hello world!"))
            })
        })?
        .run()
        .await
}
