#[macro_use]
extern crate rocket;

use chrono::{offset::Utc, DateTime};
use rocket::form::{self, DataField, Form, FromForm, FromFormField, ValueField};
use rocket::fs::{NamedFile, TempFile};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::content::RawHtml;
use rocket::{Build, Rocket};
use rocket_db_pools::{
    sqlx::{self, FromRow, PgPool},
    Connection, Database,
};
use uuid::Uuid;

#[derive(Debug, FromRow)]
struct OurDateTime(DateTime<Utc>);
#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}

#[derive(FromForm)]
struct Pagination {
    cursor: OurDateTime,
    limit: usize,
}

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
enum UserStatus {
    Inactive = 0,
    Active = 1,
}

#[derive(Debug, FromRow, FromForm)]
struct User {
    uuid: Uuid,
    username: String,
    email: String,
    password_hash: Vec<u8>,
    description: String,
    status: UserStatus,
    created_at: OurDateTime,
    updated_at: OurDateTime,
}

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
enum PostType {
    Text = 0,
    Photo = 1,
    Video = 2,
}

#[derive(FromForm)]
struct Post<'r> {
    uuid: Uuid,
    user_uuid: Uuid,
    post_type: PostType,
    content: String,
    upload_data: TempFile<'r>,
}

trait DisplayPostContent {
    fn raw_html() -> String;
}

struct TextPost<'r>(Post<'r>);
impl<'r> DisplayPostContent for TextPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

struct PhotoPost<'r>(Post<'r>);
impl<'r> DisplayPostContent for PhotoPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

struct VideoPost<'r>(Post<'r>);
impl<'r> DisplayPostContent for VideoPost<'r> {
    fn raw_html() -> String {
        todo!("will implement later")
    }
}

type HtmlResponse = Result<RawHtml<String>, Status>;

#[derive(Database)]
#[database("main_connection")]
struct DBConnection(PgPool);

#[get("/users/<uuid>", format = "text/html")]
async fn get_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users?<pagination>", format = "text/html")]
async fn get_users(
    mut db: Connection<DBConnection>,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/new", format = "text/html")]
async fn new_user(mut db: Connection<DBConnection>) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users", format = "text/html", data = "<user>")]
async fn create_user(mut db: Connection<DBConnection>, user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/edit/<uuid>", format = "text/html", rank = 1)]
async fn edit_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[put("/users/<uuid>", format = "text/html", data = "<user>")]
async fn put_user(mut db: Connection<DBConnection>, uuid: &str, user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[patch("/users/<uuid>", format = "text/html", data = "<user>")]
async fn patch_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<uuid>", format = "text/html")]
async fn delete_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
async fn get_post(mut db: Connection<DBConnection>, user_uuid: &str, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/<user_uuid>/posts", format = "text/html", rank = 2)]
async fn get_posts(mut db: Connection<DBConnection>, user_uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users/<user_uuid>/posts", format = "text/html", data = "<upload>")]
async fn create_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    upload: Form<Post<'r>>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
async fn delete_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/<filename>")]
async fn assets(filename: &str) -> NamedFile {
    todo!("will implement later")
}

#[catch(404)]
fn not_found(req: &Request) -> RawHtml<String> {
    todo!("will implement later")
}

#[catch(422)]
fn unprocessable_entity(req: &Request) -> RawHtml<String> {
    todo!("will implement later")
}

#[catch(500)]
fn internal_server_error(req: &Request) -> RawHtml<String> {
    todo!("will implement later")
}

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(DBConnection::init())
        .mount(
            "/",
            routes![
                get_user,
                get_users,
                new_user,
                create_user,
                edit_user,
                put_user,
                patch_user,
                delete_user,
                get_post,
                get_posts,
                create_post,
                delete_post,
            ],
        )
        .mount("/assets", routes![assets])
        .register(
            "/",
            catchers![not_found, unprocessable_entity, internal_server_error],
        )
}
