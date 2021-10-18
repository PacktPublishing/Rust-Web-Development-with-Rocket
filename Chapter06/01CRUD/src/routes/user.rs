use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, user::User};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket_db_pools::Connection;

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
pub async fn get_user(db: Connection<DBConnection>, uuid: &str) -> HtmlResponse {
    let user = User::find(db, uuid).await.map_err(|_| Status::NotFound)?;
    Ok(RawHtml(
        [USER_HTML_PREFIX, &user.to_html_string(), USER_HTML_SUFFIX].join(""),
    ))
}

#[get("/users?<pagination>", format = "text/html")]
pub async fn get_users(
    db: Connection<DBConnection>,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    let users = User::find_all(db, pagination)
        .await
        .map_err(|_| Status::NotFound)?;
    let mut html_string: Vec<String> = Vec::new();
    html_string.push(USER_HTML_PREFIX.to_string());
    let users_iter = users.iter();
    for user in users_iter {
        html_string.push(user.to_html_string());
    }
    html_string.push(USER_HTML_SUFFIX.to_string());
    Ok(RawHtml(html_string.join("")))
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(mut _db: Connection<DBConnection>) -> HtmlResponse {
    Ok(RawHtml(
        [
            USER_HTML_PREFIX,
            r#"<form accept-charset="UTF-8" action="/users" autocomplete="off" method="POST">
    <div>
        <label for="username">Username:</label>
        <input name="username" type="text"/>
    </div>
    <div>
        <label for="email">Email:</label>
        <input name="email" type="email"/>
    </div>
    <button type="submit" value="Submit">Submit</button>
</form>"#,
            USER_HTML_SUFFIX,
        ]
        .join(""),
    ))
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
