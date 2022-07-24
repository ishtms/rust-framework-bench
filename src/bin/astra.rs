use astra::{Body, Response, Server};

fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    Server::bind(format!("127.0.0.1:{}", port_number))
        .serve(service)
        .unwrap()
}

fn service(_req: astra::Request) -> Response {
    Response::new(Body::new("Hello, World!"))
}
/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
