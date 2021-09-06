// this will not compile
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

struct S;

#[get("/<id>")]
fn process(id: S) { /* ... */
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![process])
}
