use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello world!" }));
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port_number));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
