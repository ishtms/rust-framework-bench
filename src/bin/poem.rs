use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler]
fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let app = Route::new().at("/", get(hello));
    Server::new(TcpListener::bind(format!("127.0.0.1:{}", port_number)))
        .name("hello-world")
        .run(app)
        .await
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
