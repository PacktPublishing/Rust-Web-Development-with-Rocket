#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/<rank>", rank = 1)]
fn first(rank: u8) -> String {
    let result = rank + 10;
    format!("Your rank is, {}!", result)
}

#[get("/<name>", rank = 2)]
fn second(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![first, second])
}
