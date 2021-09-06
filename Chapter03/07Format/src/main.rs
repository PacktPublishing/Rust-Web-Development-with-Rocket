#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/get", format = "text/plain")]
fn get() -> &'static str {
    "GET Request"
}

#[post("/post", format = "form")]
fn post() -> &'static str {
    "POST Request"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![get, post])
}
