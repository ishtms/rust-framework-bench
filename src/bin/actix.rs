use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("{name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello world!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 3004))?
    .run()
    .await
}
