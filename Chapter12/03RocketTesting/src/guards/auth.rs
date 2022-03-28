use crate::fairings::db::DBConnection;
use crate::models::user::User;
use crate::states::JWToken;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::Serialize;
use rocket_db_pools::{sqlx::Acquire, Connection};
use sha2::Sha256;
use std::collections::BTreeMap;

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

impl CurrentUser {
    pub fn is(&self, uuid: &str) -> bool {
        self.user.uuid.to_string() == uuid
    }

    pub fn is_not(&self, uuid: &str) -> bool {
        !self.is(uuid)
    }
}

pub struct APIUser {
    pub user: User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for APIUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let error = || Outcome::Failure((Status::Unauthorized, ()));
        let parsed_header = req.headers().get_one("Authorization");
        if parsed_header.is_none() {
            return error();
        }
        let token_str = parsed_header.unwrap();
        let parsed_secret = req.rocket().state::<JWToken>();
        if parsed_secret.is_none() {
            return error();
        }
        let secret = &parsed_secret.unwrap().secret;
        let parsed_key: Result<Hmac<Sha256>, _> = Hmac::new_from_slice(secret.as_bytes());
        if parsed_key.is_err() {
            return error();
        }
        let key = parsed_key.unwrap();
        let parsed_token: Result<Token<Header, BTreeMap<String, String>, _>, _> =
            token_str.verify_with_key(&key);
        if parsed_token.is_err() {
            return error();
        }
        let token = parsed_token.unwrap();
        let claims = token.claims();
        let parsed_user_uuid = claims.get("user_uuid");
        if parsed_user_uuid.is_none() {
            return error();
        }
        let user_uuid = parsed_user_uuid.unwrap();
        let parsed_db = req.guard::<Connection<DBConnection>>().await;
        if !parsed_db.is_success() {
            return error();
        }
        let mut db = parsed_db.unwrap();
        let parsed_connection = db.acquire().await;
        if parsed_connection.is_err() {
            return error();
        }
        let connection = parsed_connection.unwrap();
        let found_user = User::find(connection, &user_uuid).await;
        if found_user.is_err() {
            return error();
        }
        let user = found_user.unwrap();
        Outcome::Success(APIUser { user })
    }
}
