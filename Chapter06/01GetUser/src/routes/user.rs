use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, user::User};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket_db_pools::{sqlx::Acquire, Connection};

const USER_HTML_PREFIX: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>Our Application User</title>
</head>
<body>"#;

const USER_HTML_SUFFIX: &str = r#"</body>
</html>"#;

#[get("/users/<uuid>", format = "text/html")]
pub async fn get_user(mut db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, uuid)
        .await
        .map_err(|_| Status::NotFound)?;
    let mut html_string = String::from(USER_HTML_PREFIX);
    html_string.push_str(&user.to_html_string());
    html_string
        .push_str(format!(r#"<a href="/users/edit/{}">Edit User</a><br/>"#, user.uuid).as_ref());
    html_string.push_str(r#"<a href="/users">User List</a>"#);
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
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
