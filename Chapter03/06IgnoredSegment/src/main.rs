#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/<_>")]
fn index() {}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
