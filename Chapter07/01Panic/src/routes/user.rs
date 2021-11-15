use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{
    pagination::Pagination,
    user::{EditedUser, NewUser, User},
};
use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{content::RawHtml, Flash, Redirect};
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
pub async fn get_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    flash: Option<FlashMessage<'_>>,
) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, uuid)
        .await
        .map_err(|_| Status::NotFound)?;
    let mut html_string = String::from(USER_HTML_PREFIX);
    if flash.is_some() {
        html_string.push_str(flash.unwrap().message());
    }
    html_string.push_str(&user.to_html_string());
    html_string
        .push_str(format!(r#"<a href="/users/edit/{}">Edit User</a><br/>"#, user.uuid).as_ref());
    html_string.push_str(
        format!(
            r#"<form accept-charset="UTF-8" action="/users/delete/{}" autocomplete="off" method="POST"><button type="submit" value="Submit">Delete</button></form>"#, user.uuid
        )
        .as_ref());
    html_string.push_str(r#"<a href="/users">User List</a>"#);
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[get("/users?<pagination>", format = "text/html")]
pub async fn get_users(
    mut db: Connection<DBConnection>,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    let (users, new_pagination) = User::find_all(&mut db, pagination)
        .await
        .map_err(|_| Status::NotFound)?;
    let mut html_string = String::from(USER_HTML_PREFIX);
    for user in users.iter() {
        html_string.push_str(&user.to_html_string());
        html_string
            .push_str(format!(r#"<a href="/users/{}">See User</a><br/>"#, user.uuid).as_ref());
        html_string.push_str(
            format!(r#"<a href="/users/edit/{}">Edit User</a><br/>"#, user.uuid).as_ref(),
        );
    }
    if let Some(pg) = new_pagination {
        html_string.push_str(
            format!(
                r#"<a href="/users?pagination.next={}&pagination.limit={}">Next</a><br/>"#,
                &(pg.next.0).timestamp_nanos(),
                &pg.limit,
            )
            .as_ref(),
        );
    }
    html_string.push_str(r#"<a href="/users/new">New user</a>"#);
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(flash: Option<FlashMessage<'_>>) -> HtmlResponse {
    let mut html_string = String::from(USER_HTML_PREFIX);
    if flash.is_some() {
        html_string.push_str(flash.unwrap().message());
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
        <label for="password_confirmation">Password Confirmation:</label>
        <input name="password_confirmation" type="password"/>
    </div>
    <div>
        <label for="description">Tell us a little bit more about yourself:</label>
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
    data = "<user_context>"
)]
pub async fn create_user<'r>(
    mut db: Connection<DBConnection>,
    user_context: Form<Contextual<'r, NewUser<'r>>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if user_context.value.is_none() {
        let error_message = format!(
            "<div>{}</div>",
            user_context
                .context
                .errors()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("<br/>")
        );
        return Err(Flash::error(Redirect::to("/users/new"), error_message));
    }
    let new_user = user_context.value.as_ref().unwrap();
    let connection = db.acquire().await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "<div>Something went wrong when creating user</div>",
        )
    })?;
    let user = User::create(connection, new_user).await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "<div>Something went wrong when creating user</div>",
        )
    })?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}", user.uuid)),
        "<div>Successfully created user</div>",
    ))
}

#[get("/users/edit/<uuid>", format = "text/html")]
pub async fn edit_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    flash: Option<FlashMessage<'_>>,
) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, uuid)
        .await
        .map_err(|_| Status::NotFound)?;
    let mut html_string = String::from(USER_HTML_PREFIX);
    if flash.is_some() {
        html_string.push_str(flash.unwrap().message());
    }
    html_string.push_str(
        format!(
            r#"<form accept-charset="UTF-8" action="/users/{}" autocomplete="off" method="POST">
    <input type="hidden" name="_METHOD" value="PUT"/>
    <div>
        <label for="username">Username:</label>
        <input name="username" type="text" value="{}"/>
    </div>
    <div>
        <label for="email">Email:</label>
        <input name="email" type="email" value="{}"/>
    </div>
    <div>
        <label for="old_password">Old password:</label>
        <input name="old_password" type="password"/>
    </div>
    <div>
        <label for="password">New password:</label>
        <input name="password" type="password"/>
    </div>
    <div>
        <label for="password_confirmation">Password Confirmation:</label>
        <input name="password_confirmation" type="password"/>
    </div>
    <div>
        <label for="description">Tell us a little bit more about yourself:</label>
        <textarea name="description">{}</textarea>
    </div>
    <button type="submit" value="Submit">Submit</button>
</form>"#,
            &user.uuid,
            &user.username,
            &user.email,
            &user.description.unwrap_or_else(|| "".to_string()),
        )
        .as_ref(),
    );
    html_string.push_str(USER_HTML_SUFFIX);
    Ok(RawHtml(html_string))
}

#[post(
    "/users/<uuid>",
    format = "application/x-www-form-urlencoded",
    data = "<user_context>"
)]
pub async fn update_user<'r>(
    db: Connection<DBConnection>,
    uuid: &str,
    user_context: Form<Contextual<'r, EditedUser<'r>>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if user_context.value.is_none() {
        let error_message = format!(
            "<div>{}</div>",
            user_context
                .context
                .errors()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("<br/>")
        );
        return Err(Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            error_message,
        ));
    }
    let user_value = user_context.value.as_ref().unwrap();
    match user_value.method {
        "PUT" => put_user(db, uuid, user_context).await,
        "PATCH" => patch_user(db, uuid, user_context).await,
        _ => Err(Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            "<div>Something went wrong when updating user</div>",
        )),
    }
}

#[put(
    "/users/<uuid>",
    format = "application/x-www-form-urlencoded",
    data = "<user_context>"
)]
pub async fn put_user<'r>(
    mut db: Connection<DBConnection>,
    uuid: &str,
    user_context: Form<Contextual<'r, EditedUser<'r>>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let user_value = user_context.value.as_ref().unwrap();
    let user = User::update(&mut db, uuid, user_value).await.map_err(|_| {
        Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            "<div>Something went wrong when updating user</div>",
        )
    })?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}", user.uuid)),
        "<div>Successfully updated user</div>",
    ))
}

#[patch(
    "/users/<uuid>",
    format = "application/x-www-form-urlencoded",
    data = "<user_context>"
)]
pub async fn patch_user<'r>(
    db: Connection<DBConnection>,
    uuid: &str,
    user_context: Form<Contextual<'r, EditedUser<'r>>>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    put_user(db, uuid, user_context).await
}

#[post("/users/delete/<uuid>", format = "application/x-www-form-urlencoded")]
pub async fn delete_user_entry_point(
    db: Connection<DBConnection>,
    uuid: &str,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    delete_user(db, uuid).await
}

#[delete("/users/<uuid>", format = "application/x-www-form-urlencoded")]
pub async fn delete_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let connection = db.acquire().await.map_err(|_| {
        Flash::error(
            Redirect::to("/users"),
            "<div>Something went wrong when deleting user</div>",
        )
    })?;
    User::destroy(connection, uuid).await.map_err(|_| {
        Flash::error(
            Redirect::to("/users"),
            "<div>Something went wrong when deleting user</div>",
        )
    })?;
    Ok(Flash::success(
        Redirect::to("/users"),
        "<div>Successfully deleted user</div>",
    ))
}
