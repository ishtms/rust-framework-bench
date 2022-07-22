use std::net::SocketAddr;
use viz::{get, Body, Request, Result, Router, Server, ServiceMaker};

async fn index(_: Request<Body>) -> Result<&'static str> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    Server::bind(&SocketAddr::from(([127, 0, 0, 1], port_number)))
        .tcp_nodelay(true)
        .serve(ServiceMaker::from(app))
        .await
        .unwrap()
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
