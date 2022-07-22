use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello world!")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let addr = ([127, 0, 0, 1], port_number).into();
    let server = Server::bind(&addr).serve(make_svc);
    server.await?;

    Ok(())
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
