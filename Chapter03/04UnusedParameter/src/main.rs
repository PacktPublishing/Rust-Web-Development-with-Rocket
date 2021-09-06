// this will not compile
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/<id>")]
fn process() { /* ... */
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![process])
}
