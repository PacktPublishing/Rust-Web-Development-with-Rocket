#[macro_use]
extern crate rocket;

use rocket::{form::Form, Build, Rocket};

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

#[post("/post", data = "<data>")]
fn post(data: Form<Filters>) -> &'static str {
    "POST Request"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![post])
}
