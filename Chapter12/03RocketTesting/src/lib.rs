#[macro_use]
extern crate rocket;

use crate::fairings::{csrf::Csrf, db::DBConnection};
use crate::models::worker::Message;
use crate::routes::{api, post, session, user};
use crate::states::JWToken;
use crate::workers::video::process_video;
use log::LevelFilter;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Handle;

pub mod catchers;
pub mod errors;
pub mod fairings;
pub mod guards;
pub mod models;
pub mod routes;
pub mod states;
pub mod traits;
pub mod workers;

#[derive(Deserialize)]
pub struct Config {
    databases: Databases,
    jwt_secret: String,
}

#[derive(Deserialize)]
struct Databases {
    main_connection: MainConnection,
}

#[derive(Deserialize)]
struct MainConnection {
    url: String,
}

impl Config {
    pub fn get_database_url(&self) -> String {
        self.databases.main_connection.url.clone()
    }
}

fn setup_logger() {
    let (level, logger) = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date}] [{level}][{target}] [{message}]",
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                target = record.target(),
                level = record.level(),
                message = message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::log_file("logs/application.log")
                .unwrap_or_else(|_| panic!("Cannot open logs/application.log")),
        )
        .into_log();
    async_log::Logger::wrap(logger, || 0).start(level).unwrap();
}

pub async fn setup_rocket() -> Rocket<Build> {
    setup_logger();
    let (tx, rx) = flume::bounded::<Message>(5);

    let our_rocket = rocket::build()
        .attach(DBConnection::init())
        .attach(Template::fairing())
        .attach(Csrf::new())
        .manage(tx)
        .mount(
            "/",
            routes![
                user::get_user,
                user::get_users,
                user::new_user,
                user::create_user,
                user::edit_user,
                user::update_user,
                user::put_user,
                user::patch_user,
                user::delete_user,
                user::delete_user_entry_point,
                post::get_post,
                post::get_posts,
                post::create_post,
                post::delete_post,
                routes::home,
                routes::shutdown,
                session::new,
                session::create,
                session::delete,
            ],
        )
        .mount("/assets", FileServer::from(relative!("static")))
        .mount(
            "/api",
            routes![api::users, api::login, api::authenticated_users,],
        )
        .register(
            "/",
            catchers![
                catchers::bad_request,
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_server_error
            ],
        );
    let config: Config = our_rocket
        .figment()
        .extract()
        .expect("Incorrect Rocket.toml configuration");

    let jwt_secret = JWToken {
        secret: String::from(config.jwt_secret.clone()),
    };

    let final_rocket = our_rocket.manage(jwt_secret);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.databases.main_connection.url)
        .await
        .expect("Failed to connect to database");

    tokio::task::spawn_blocking(move || loop {
        let wm = rx.recv().unwrap();
        let handle = Handle::current();
        let get_connection = async { (&pool).acquire().await.unwrap() };
        let mut connection = handle.block_on(get_connection);
        let _ = process_video(&mut connection, wm);
    });
    final_rocket
}
