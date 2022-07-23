use thruster::BasicContext as Context;
use thruster::{
    m, middleware_fn, App, MiddlewareNext, MiddlewareResult, Request, Server, ThrusterServer,
};

#[middleware_fn]
async fn hello(mut ctx: Context, _next: MiddlewareNext<Context>) -> MiddlewareResult<Context> {
    ctx.body("Hello, world!");
    Ok(ctx)
}

fn main() {
    let app = App::<Request, Context, ()>::new_basic().get("/", m![hello]);
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    Server::new(app).start("127.0.0.1", port_number);
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
