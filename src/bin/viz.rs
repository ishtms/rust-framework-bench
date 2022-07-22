use std::net::SocketAddr;
use viz::{get, Body, Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request<Body>) -> Result<&'static str> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3007)))
        .tcp_nodelay(true)
        .serve(ServiceMaker::from(app))
        .await
        .unwrap()
}
