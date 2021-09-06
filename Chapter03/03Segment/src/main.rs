#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/<id>")]
fn process(id: u8) { /* ... */
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![process])
}
