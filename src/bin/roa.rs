use roa::preload::*;
use roa::App;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    let app = App::new().end("Hello, World");
    app.listen(format!("127.0.0.1:{}", port_number), |_| {})?
        .await?;
    Ok(())
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
