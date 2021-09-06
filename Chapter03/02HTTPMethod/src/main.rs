#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

#[get("/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) {
    /* ... */
}

#[get("/users?<grade>&<filters..>")]
fn users(grade: u8, filters: Filters) {
    /* ... */
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}
