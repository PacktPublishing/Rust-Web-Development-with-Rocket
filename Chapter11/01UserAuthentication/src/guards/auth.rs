use crate::fairings::db::DBConnection;
use crate::models::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket_db_pools::{sqlx::Acquire, Connection};

pub const LOGIN_COOKIE_NAME: &str = "user_uuid";

#[derive(Serialize)]
pub struct CurrentUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error = Outcome::Failure((Status::Unauthorized, ()));
        let parsed_cookie = req.cookies().get_private(LOGIN_COOKIE_NAME);
        if parsed_cookie.is_none() {
            return error;
        }
        let cookie = parsed_cookie.unwrap();
        let uuid = cookie.value();
        let parsed_db = req.guard::<Connection<DBConnection>>().await;
        if !parsed_db.is_success() {
            return error;
        }
        let mut db = parsed_db.unwrap();
        let parsed_connection = db.acquire().await;
        if parsed_connection.is_err() {
            return error;
        }
        let connection = parsed_connection.unwrap();
        let found_user = User::find(connection, uuid).await;
        if found_user.is_err() {
            return error;
        }
        let user = found_user.unwrap();
        Outcome::Success(CurrentUser { user })
    }
}
