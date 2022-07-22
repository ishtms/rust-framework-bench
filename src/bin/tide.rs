#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();

    app.at("/").get(|_| async { Ok("Hello, world!") });

    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    app.listen(format!("127.0.0.1:{}", port_number)).await?;
    Ok(())
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
