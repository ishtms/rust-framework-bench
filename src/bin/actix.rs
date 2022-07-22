use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("{name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello world!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", port_number))?
    .run()
    .await
}

/// retrieve port number from the `main.rs` script
/// that is assigned in `config.json`
fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
