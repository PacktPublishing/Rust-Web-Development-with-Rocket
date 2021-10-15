use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, user::User};
use rocket::form::Form;
use rocket_db_pools::Connection;

#[get("/users/<_uuid>", format = "text/html")]
pub async fn get_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users?<_pagination>", format = "text/html")]
pub async fn get_users(
    mut _db: Connection<DBConnection>,
    _pagination: Option<Pagination>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(mut _db: Connection<DBConnection>) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users", format = "text/html", data = "<_user>")]
pub async fn create_user(mut _db: Connection<DBConnection>, _user: Form<User>) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/edit/<_uuid>", format = "text/html")]
pub async fn edit_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}

#[put("/users/<_uuid>", format = "text/html", data = "<_user>")]
pub async fn put_user(
    mut _db: Connection<DBConnection>,
    _uuid: &str,
    _user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[patch("/users/<_uuid>", format = "text/html", data = "<_user>")]
pub async fn patch_user(
    mut _db: Connection<DBConnection>,
    _uuid: &str,
    _user: Form<User>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<_uuid>", format = "text/html")]
pub async fn delete_user(mut _db: Connection<DBConnection>, _uuid: &str) -> HtmlResponse {
    todo!("will implement later")
}
