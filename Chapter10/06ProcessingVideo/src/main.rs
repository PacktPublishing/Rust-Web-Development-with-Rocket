#[macro_use]
extern crate rocket;

use crossbeam::channel;
use log::LevelFilter;
use our_application::catchers;
use our_application::fairings::{csrf::Csrf, db::DBConnection};
use our_application::models::worker::Message;
use our_application::routes::{self, post, user};
use our_application::workers::photo::process_photo;
use our_application::workers::video::process_video;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::Deserialize;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use sqlx::postgres::PgPoolOptions;
use tokio::runtime::Handle;

#[derive(Deserialize)]
struct Config {
    databases: Databases,
}

#[derive(Deserialize)]
struct Databases {
    main_connection: MainConnection,
}

#[derive(Deserialize)]
struct MainConnection {
    url: String,
}

#[launch]
async fn rocket() -> Rocket<Build> {
    setup_logger();
    let our_rocket = rocket::build();
    let config: Config = our_rocket
        .figment()
        .extract()
        .expect("Incorrect Rocket.toml configuration");

    let (tx, rx) = channel::bounded::<Message>(5);
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
        if wm.is_bitmap() {
            let _ = process_photo(&mut connection, wm.uuid, wm.orig_filename, wm.dest_filename);
        } else if wm.is_video() {
            let _ = process_video(&mut connection, wm.uuid, wm.orig_filename, wm.dest_filename);
        };
    });

    our_rocket
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
                routes::shutdown,
            ],
        )
        .mount("/assets", FileServer::from(relative!("static")))
        .register(
            "/",
            catchers![
                catchers::bad_request,
                catchers::not_found,
                catchers::unprocessable_entity,
                catchers::internal_server_error
            ],
        )
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
