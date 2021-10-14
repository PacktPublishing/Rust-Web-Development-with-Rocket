use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, user::User};
use rocket::form::Form;
use rocket_db_pools::Connection;

#[get("/users/<uuid>", format = "text/html")]
pub async fn get_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users?<pagination>", format = "text/html")]
pub async fn get_users(
    mut db: Connection<DBConnection>,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(mut db: Connection<DBConnection>) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users", format = "text/html", data = "<user>")]
pub async fn create_user(mut db: Connection<DBConnection>, user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/edit/<uuid>", format = "text/html")]
pub async fn edit_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[put("/users/<uuid>", format = "text/html", data = "<user>")]
pub async fn put_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[patch("/users/<uuid>", format = "text/html", data = "<user>")]
pub async fn patch_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<uuid>", format = "text/html")]
pub async fn delete_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}
