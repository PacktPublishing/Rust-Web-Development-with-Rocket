use super::HtmlResponse;
use crate::fairings::csrf::Token as CsrfToken;
use crate::fairings::db::DBConnection;
use crate::guards::auth::CurrentUser;
use crate::models::{
    pagination::Pagination,
    user::{EditedUser, NewUser, User},
};
use rocket::form::{Contextual, Form};
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};

#[get("/users/<uuid>", format = "text/html")]
pub async fn get_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    flash: Option<FlashMessage<'_>>,
    current_user: Option<CurrentUser>,
) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, uuid).await.map_err(|e| e.status)?;
    let flash_message = flash.map(|fm| String::from(fm.message()));
    let context = context! {
        user,
        current_user,
        flash: flash_message,
    };
    Ok(Template::render("users/show", context))
}

#[get("/users?<pagination>", format = "text/html")]
pub async fn get_users(
    mut db: Connection<DBConnection>,
    pagination: Option<Pagination>,
    current_user: Option<CurrentUser>,
) -> HtmlResponse {
    let (users, new_pagination) = User::find_all(&mut db, pagination)
        .await
        .map_err(|e| e.status)?;
    let context = context! {
        users,
        current_user,
        pagination: new_pagination.map(|pg|pg.to_context()),
    };
    Ok(Template::render("users/index", context))
}

#[get("/users/new", format = "text/html")]
pub async fn new_user(flash: Option<FlashMessage<'_>>, csrf_token: CsrfToken) -> HtmlResponse {
    let flash_string = flash
        .map(|fl| format!("{}", fl.message()))
        .unwrap_or_else(|| "".to_string());
    let context = context! {
        edit: false,
        form_url: "/users",
        legend: "New User",
        flash: flash_string,
        csrf_token,
    };
    Ok(Template::render("users/form", context))
}

#[post(
    "/users",
    format = "application/x-www-form-urlencoded",
    data = "<user_context>"
)]
pub async fn create_user<'r>(
    mut db: Connection<DBConnection>,
    user_context: Form<Contextual<'r, NewUser<'r>>>,
    csrf_token: CsrfToken,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if user_context.value.is_none() {
        let error_message = user_context
            .context
            .errors()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("<br/>");
        return Err(Flash::error(Redirect::to("/users/new"), error_message));
    }
    let new_user = user_context.value.as_ref().unwrap();
    csrf_token
        .verify(&new_user.authenticity_token)
        .map_err(|_| {
            Flash::error(
                Redirect::to("/users/new"),
                "Something went wrong when creating user",
            )
        })?;
    let connection = db.acquire().await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "Something went wrong when creating user",
        )
    })?;
    let user = User::create(connection, new_user).await.map_err(|_| {
        Flash::error(
            Redirect::to("/users/new"),
            "Something went wrong when creating user",
        )
    })?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}", user.uuid)),
        "Successfully created user",
    ))
}

#[get("/users/edit/<uuid>", format = "text/html")]
pub async fn edit_user(
    uuid: &str,
    flash: Option<FlashMessage<'_>>,
    csrf_token: CsrfToken,
    current_user: CurrentUser,
) -> HtmlResponse {
    if current_user.is_not(uuid) {
        return Err(Status::Unauthorized);
    }
    let flash_string = flash
        .map(|fl| format!("{}", fl.message()))
        .unwrap_or_else(|| "".to_string());
    let context = context! {
        form_url: format!("/users/{}", uuid),
        edit: true,
        legend: "Edit User",
        flash: flash_string,
        current_user,
        csrf_token,
    };

    Ok(Template::render("users/form", context))
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
    csrf_token: CsrfToken,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    if user_context.value.is_none() {
        let error_message = user_context
            .context
            .errors()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("<br/>");
        return Err(Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            error_message,
        ));
    }
    let user_value = user_context.value.as_ref().unwrap();
    match user_value.method {
        "PUT" => put_user(db, uuid, user_context, csrf_token, current_user).await,
        "PATCH" => patch_user(db, uuid, user_context, csrf_token, current_user).await,
        _ => Err(Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            "Something went wrong when updating user",
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
    csrf_token: CsrfToken,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let update_error = || {
        Flash::error(
            Redirect::to(format!("/users/edit/{}", uuid)),
            "Something went wrong when updating user",
        )
    };
    let user_value = user_context.value.as_ref().unwrap();
    csrf_token
        .verify(&user_value.authenticity_token)
        .map_err(|_| update_error())?;
    if current_user.is_not(uuid) {
        return Err(update_error());
    }
    let user = User::update(&mut db, uuid, user_value)
        .await
        .map_err(|_| update_error())?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}", user.uuid)),
        "Successfully updated user",
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
    csrf_token: CsrfToken,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    put_user(db, uuid, user_context, csrf_token, current_user).await
}

#[post(
    "/users/delete/<uuid>",
    format = "application/x-www-form-urlencoded",
    rank = 2
)]
pub async fn delete_user_entry_point(
    db: Connection<DBConnection>,
    uuid: &str,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    delete_user(db, uuid, current_user).await
}

#[delete("/users/<uuid>", format = "application/x-www-form-urlencoded")]
pub async fn delete_user(
    mut db: Connection<DBConnection>,
    uuid: &str,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let delete_error = || {
        Flash::error(
            Redirect::to("/users"),
            "Something went wrong when deleting user",
        )
    };
    if current_user.is_not(uuid) {
        return Err(delete_error());
    }
    let connection = db.acquire().await.map_err(|_| delete_error())?;
    User::destroy(connection, uuid)
        .await
        .map_err(|_| delete_error())?;
    Ok(Flash::success(
        Redirect::to("/users"),
        "Successfully deleted user",
    ))
}
