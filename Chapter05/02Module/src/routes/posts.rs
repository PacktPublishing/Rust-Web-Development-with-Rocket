use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, post::Post};
use rocket::form::Form;
use rocket_db_pools::Connection;

#[get("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
pub async fn get_post(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
) -> HtmlResponse {
    todo!("will implement later")
}

#[get("/users/<user_uuid>/posts?<pagination>", format = "text/html")]
pub async fn get_posts(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users/<user_uuid>/posts", format = "text/html", data = "<upload>")]
pub async fn create_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    upload: Form<Post<'r>>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[post("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
pub async fn delete_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
) -> HtmlResponse {
    todo!("will implement later")
}
