#[macro_use]
extern crate rocket;

use our_application::catchers;
use our_application::routes::{self, posts, users};
use our_application::fairings::db::DBConnection;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DBConnection::init())
        .mount(
            "/",
            routes![
                users::get_user,
                users::get_users,
                users::new_user,
                users::create_user,
                users::edit_user,
                users::put_user,
                users::patch_user,
                users::delete_user,
                posts::get_post,
                posts::get_posts,
                posts::create_post,
                posts::delete_post,
            ],
        )
        .mount("/assets", routes![routes::assets])
        .register(
            "/",
            catchers![
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_server_error
            ],
        )
}
