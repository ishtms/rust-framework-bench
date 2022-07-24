#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let mut server = Nickel::new();
    server.get("**", middleware!("Hello, world!"));
    server.listen(format!("127.0.0.1:{}", port_number)).unwrap();
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
