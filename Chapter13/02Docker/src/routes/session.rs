use super::HtmlResponse;
use crate::fairings::csrf::Token as CsrfToken;
use crate::fairings::db::DBConnection;
use crate::guards::auth::LOGIN_COOKIE_NAME;
use crate::models::user::{Login, User};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};

#[get("/login", format = "text/html")]
pub async fn new<'r>(flash: Option<FlashMessage<'_>>, csrf_token: CsrfToken) -> HtmlResponse {
    let flash_string = flash
        .map(|fl| format!("{}", fl.message()))
        .unwrap_or_else(|| "".to_string());
    let context = context! {
        flash: flash_string,
        csrf_token: csrf_token,
    };
    Ok(Template::render("sessions/new", context))
}

#[post(
    "/login",
    format = "application/x-www-form-urlencoded",
    data = "<login_context>"
)]
pub async fn create<'r>(
    mut db: Connection<DBConnection>,
    login_context: Form<Contextual<'r, Login<'r>>>,
    csrf_token: CsrfToken,
    cookies: &CookieJar<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let login_error = || Flash::error(Redirect::to("/login"), "Cannot login");
    if login_context.value.is_none() {
        return Err(login_error());
    }
    let login = login_context.value.as_ref().unwrap();
    csrf_token
        .verify(&login.authenticity_token)
        .map_err(|_| login_error())?;
    let connection = db.acquire().await.map_err(|_| login_error())?;
    let user = User::find_by_login(connection, login)
        .await
        .map_err(|_| login_error())?;
    cookies.add_private(Cookie::new(LOGIN_COOKIE_NAME, user.uuid.to_string()));

    Ok(Flash::success(Redirect::to("/users"), "Login successfully"))
}

#[post("/logout", format = "application/x-www-form-urlencoded")]
pub async fn delete(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named(LOGIN_COOKIE_NAME));
    Flash::success(Redirect::to("/users"), "Logout successfully")
}
