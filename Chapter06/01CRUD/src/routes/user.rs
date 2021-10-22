use super::{HtmlResponse, RedirectResponse};
use crate::fairings::db::DBConnection;
use crate::models::{
    pagination::Pagination,
    user::{NewUser, User},
};
use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{content::RawHtml, Flash, Redirect};
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
    let mut html_string = String::from(USER_HTML_PREFIX);
    let users_iter = users.iter();
    for user in users_iter {
        html_string.push_str(&user.to_html_string());
    }
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[get("/users/new", format = "text/html")]
pub async fn new_user<'r>(flash: Option<FlashMessage<'r>>) -> HtmlResponse {
    let mut html_string = String::from(USER_HTML_PREFIX);
    if flash.is_some() {
        html_string.push_str(format!("<div>{}</div>", flash.unwrap().message()).as_ref())
    }
    html_string.push_str(
        r#"<form accept-charset="UTF-8" action="/users" autocomplete="off" method="POST">
    <div>
        <label for="username">Username:</label>
        <input name="username" type="text"/>
    </div>
    <div>
        <label for="email">Email:</label>
        <input name="email" type="email"/>
    </div>
    <div>
        <label for="password">Password:</label>
        <input name="password" type="password"/>
    </div>
    <div>
        <label for="password_confirmation">Confirm password:</label>
        <input name="password_confirmation" type="password"/>
    </div>
    <div>
        <label for="description">Tell us a little bit about yourself:</label>
        <textarea name="description"></textarea>
    </div>
    <button type="submit" value="Submit">Submit</button>
</form>"#,
    );
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[post(
    "/users",
    format = "application/x-www-form-urlencoded",
    data = "<user_context>",
    rank = 1
)]
pub async fn create_user<'r>(
    db: Connection<DBConnection>,
    user_context: Form<Contextual<'r, NewUser<'r>>>,
) -> RedirectResponse {
    match user_context.value {
        Some(ref new_user) => Ok(RawHtml(
            [
                USER_HTML_PREFIX.to_string(),
                format!("{:?}", new_user),
                USER_HTML_SUFFIX.to_string(),
            ]
            .join(""),
        )),
        None => {
            let mut error_message = String::new();
            user_context
                .context
                .errors()
                .filter(|err| err.name.is_some())
                .for_each(|err| error_message.push_str(err.value.as_ref().map()));
            println!("---------------------------------------{}", error_message);
            Err(Flash::error(Redirect::to("/users/new"), error_message))
        }
    }
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
