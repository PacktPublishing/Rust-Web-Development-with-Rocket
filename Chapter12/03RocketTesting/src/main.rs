#[macro_use]
extern crate rocket;

use our_application::setup_rocket;
use rocket::{Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    setup_rocket().await
}
