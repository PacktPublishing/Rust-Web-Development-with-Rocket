#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, Rocket!"
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
