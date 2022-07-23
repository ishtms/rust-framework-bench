use ntex::web::{self, middleware, App, HttpRequest};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let port_number: u16 = str::parse(get_port_number().as_str()).unwrap();

    web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind(format!("127.0.0.1:{}", port_number))?
    .run()
    .await
}

fn get_port_number() -> String {
    std::env::args().collect::<Vec<String>>()[1].clone()
}
