use salvo::prelude::*;

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let router = Router::new().get(hello);
    Server::new(TcpListener::bind(
        format!("127.0.0.1:{}", port_number).as_str(),
    ))
    .serve(router)
    .await
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
