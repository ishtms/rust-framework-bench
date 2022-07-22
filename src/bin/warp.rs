use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello world!");
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], port_number)).await;
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
