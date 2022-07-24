#[macro_use]
extern crate rocket;

// this is our get route which will be requested at the "/" location wherever it is mounted
#[get("/")]
fn say_hello() -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hello])
}
