use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(format!("127.0.0.1:{}", port_number))?
        .run()
        .await
}

async fn index() -> &'static str {
    "Hello, World!"
}
/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
