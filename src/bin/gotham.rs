use gotham::state::State;

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello, world!")
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let addr = format!("127.0.0.1:{}", port_number);
    gotham::start(addr, || Ok(say_hello)).unwrap();
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
