use super::bool_wrapper::BoolWrapper;
use super::clean_html;
use super::our_date_time::OurDateTime;
use super::pagination::{Pagination, DEFAULT_LIMIT};
use super::user_status::UserStatus;
use crate::errors::our_error::OurError;
use crate::fairings::db::DBConnection;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::offset::Utc;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use regex::Regex;
use rocket::form::{self, Error as FormError, FromForm};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::sqlx::{Acquire, FromRow, PgConnection};
use rocket_db_pools::Connection;
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;
use zxcvbn::zxcvbn;

#[derive(Debug, FromRow, FromForm, Serialize)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub description: Option<String>,
    pub status: UserStatus,
    pub created_at: OurDateTime,
    pub updated_at: OurDateTime,
}

impl User {
    pub async fn find(connection: &mut PgConnection, uuid: &str) -> Result<Self, OurError> {
        let parsed_uuid = Uuid::parse_str(uuid).map_err(OurError::from_uuid_error)?;
        let query_str = "SELECT * FROM users WHERE uuid = $1";
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await
            .map_err(OurError::from_sqlx_error)?)
    }

    pub async fn find_by_login<'r>(
        connection: &mut PgConnection,
        login: &'r Login<'r>,
    ) -> Result<Self, OurError> {
        let query_str = "SELECT * FROM users WHERE username = $1";
        let user = sqlx::query_as::<_, Self>(query_str)
            .bind(&login.username)
            .fetch_one(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        let argon2 = Argon2::default();
        verify_password(&argon2, &user.password_hash, &login.password)?;
        Ok(user)
    }

    pub async fn find_all(
        db: &mut Connection<DBConnection>,
        pagination: Option<Pagination>,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        if pagination.is_some() {
            return Self::find_all_with_pagination(db, &(pagination.unwrap())).await;
        } else {
            return Self::find_all_without_pagination(db).await;
        }
    }

    async fn find_all_without_pagination(
        db: &mut Connection<DBConnection>,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        let query_str = "SELECT * FROM users ORDER BY created_at DESC LIMIT $1";
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let users = sqlx::query_as::<_, Self>(query_str)
            .bind(DEFAULT_LIMIT as i32)
            .fetch_all(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        let mut new_pagination: Option<Pagination> = None;
        if users.len() == DEFAULT_LIMIT {
            let query_str = "SELECT EXISTS(SELECT 1 FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&users.last().unwrap().created_at)
                .fetch_one(connection)
                .await
                .map_err(OurError::from_sqlx_error)?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: users.last().unwrap().created_at.to_owned(),
                    limit: DEFAULT_LIMIT,
                });
            }
        }
        Ok((users, new_pagination))
    }

    async fn find_all_with_pagination(
        db: &mut Connection<DBConnection>,
        pagination: &Pagination,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        let query_str =
            "SELECT * FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT $2";
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let users = sqlx::query_as::<_, Self>(query_str)
            .bind(&pagination.next)
            .bind(pagination.limit as i32)
            .fetch_all(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        let mut new_pagination: Option<Pagination> = None;
        if users.len() == pagination.limit {
            let query_str = "SELECT EXISTS(SELECT 1 FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&users.last().unwrap().created_at)
                .fetch_one(connection)
                .await
                .map_err(OurError::from_sqlx_error)?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: users.last().unwrap().created_at.to_owned(),
                    limit: pagination.limit,
                });
            }
        }
        Ok((users, new_pagination))
    }

    pub async fn create<'r>(
        connection: &mut PgConnection,
        new_user: &'r NewUser<'r>,
    ) -> Result<Self, OurError> {
        let uuid = Uuid::new_v4();
        let username = &(clean_html(new_user.username));
        let description = &(new_user.description.map(|desc| clean_html(desc)));
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .map_err(|e| {
                OurError::new_internal_server_error(
                    String::from("Something went wrong"),
                    Some(Box::new(e)),
                )
            })?;

        let query_str = r#"INSERT INTO users
(uuid, username, email, password_hash, description, status)
VALUES
($1, $2, $3, $4, $5, $6)
RETURNING *"#;
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(uuid)
            .bind(username)
            .bind(new_user.email)
            .bind(password_hash.to_string())
            .bind(description)
            .bind(UserStatus::Inactive)
            .fetch_one(connection)
            .await
            .map_err(OurError::from_sqlx_error)?)
    }

    pub async fn update<'r>(
        db: &mut Connection<DBConnection>,
        uuid: &'r str,
        user: &'r EditedUser<'r>,
    ) -> Result<Self, OurError> {
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let old_user = Self::find(connection, uuid).await?;
        let now = OurDateTime(Utc::now());
        let username = &(clean_html(user.username));
        let description = &(user.description.map(|desc| clean_html(desc)));
        let mut set_strings = vec![
            "username = $1",
            "email = $2",
            "description = $3",
            "updated_at = $4",
        ];
        let mut where_string = "$5";
        let mut password_string = String::new();
        let is_with_password = !user.old_password.is_empty();
        if is_with_password {
            let argon2 = Argon2::default();
            verify_password(&argon2, &old_user.password_hash, user.old_password)?;
            let salt = SaltString::generate(&mut OsRng);
            let new_hash = argon2
                .hash_password(user.password.as_bytes(), &salt)
                .map_err(|e| {
                    OurError::new_internal_server_error(
                        String::from("Something went wrong"),
                        Some(Box::new(e)),
                    )
                })?;
            password_string.push_str(new_hash.to_string().as_ref());
            set_strings.push("password_hash = $5");
            where_string = "$6";
        }
        let query_str = format!(
            r#"UPDATE users SET {} WHERE uuid = {} RETURNING *"#,
            set_strings.join(", "),
            where_string,
        );
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let mut binded = sqlx::query_as::<_, Self>(&query_str)
            .bind(username)
            .bind(user.email)
            .bind(description)
            .bind(&now);
        if is_with_password {
            binded = binded.bind(&password_string);
        }
        let parsed_uuid = Uuid::parse_str(uuid).map_err(OurError::from_uuid_error)?;
        Ok(binded
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await
            .map_err(OurError::from_sqlx_error)?)
    }

    pub async fn destroy(connection: &mut PgConnection, uuid: &str) -> Result<(), OurError> {
        let parsed_uuid = Uuid::parse_str(uuid).map_err(OurError::from_uuid_error)?;
        let query_str = "DELETE FROM users WHERE uuid = $1";
        sqlx::query(query_str)
            .bind(parsed_uuid)
            .execute(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        Ok(())
    }

    pub fn to_html_string(&self) -> String {
        format!(
            r#"<div>UUID: {uuid}</div>
<div>Username: {username}</div>
<div>Email: {email}</div>
<div>Description: {description}</div>
<div>Status: {status}</div>
<div>Created At: {created_at}</div>
<div>Updated At: {updated_at}</div>"#,
            uuid = self.uuid,
            username = self.username,
            email = self.email,
            description = self.description.as_ref().unwrap_or(&String::from("")),
            status = self.status.to_string(),
            created_at = self.created_at.0.to_rfc3339(),
            updated_at = self.updated_at.0.to_rfc3339(),
        )
    }
}

#[derive(Debug, FromForm)]
pub struct NewUser<'r> {
    #[field(validate = len(5..20).or_else(msg!("name cannot be empty")))]
    pub username: &'r str,
    #[field(validate = validate_email().or_else(msg!("invalid email")))]
    pub email: &'r str,
    #[field(validate = validate_password().or_else(msg!("weak password")))]
    pub password: &'r str,
    #[field(validate = eq(self.password).or_else(msg!("password confirmation mismatch")))]
    pub password_confirmation: &'r str,
    #[field(default = "")]
    pub description: Option<&'r str>,
    pub authenticity_token: &'r str,
}

#[derive(Debug, FromForm)]
pub struct EditedUser<'r> {
    #[field(name = "_METHOD")]
    pub method: &'r str,
    #[field(validate = len(5..20).or_else(msg!("name cannot be empty")))]
    pub username: &'r str,
    #[field(validate = validate_email().or_else(msg!("invalid email")))]
    pub email: &'r str,
    pub old_password: &'r str,
    #[field(validate = skip_validate_password(self.old_password, self.password_confirmation))]
    pub password: &'r str,
    pub password_confirmation: &'r str,
    #[field(default = "")]
    pub description: Option<&'r str>,
    pub authenticity_token: &'r str,
}

fn validate_email(email: &str) -> form::Result<'_, ()> {
    const EMAIL_REGEX: &str = r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;
    let email_regex = Regex::new(EMAIL_REGEX).unwrap();
    if !email_regex.is_match(email) {
        return Err(FormError::validation("invalid email").into());
    }
    Ok(())
}

fn validate_password(password: &str) -> form::Result<'_, ()> {
    let entropy = zxcvbn(password, &[]);
    if entropy.is_err() || entropy.unwrap().score() < 3 {
        return Err(FormError::validation("weak password").into());
    }
    Ok(())
}

fn skip_validate_password<'v>(
    password: &'v str,
    old_password: &'v str,
    password_confirmation: &'v str,
) -> form::Result<'v, ()> {
    if old_password.is_empty() {
        return Ok(());
    }
    validate_password(password)?;
    if password.ne(password_confirmation) {
        return Err(FormError::validation("password confirmation mismatch").into());
    }
    Ok(())
}

fn verify_password(ag: &Argon2, reference: &str, password: &str) -> Result<(), OurError> {
    let reference_hash = PasswordHash::new(reference).map_err(|e| {
        OurError::new_internal_server_error(String::from("Input error"), Some(Box::new(e)))
    })?;
    Ok(ag
        .verify_password(password.as_bytes(), &reference_hash)
        .map_err(|e| {
            OurError::new_internal_server_error(
                String::from("Cannot verify password"),
                Some(Box::new(e)),
            )
        })?)
}

#[derive(FromForm)]
pub struct Login<'r> {
    pub username: &'r str,
    pub password: &'r str,
    pub authenticity_token: &'r str,
}

#[derive(Serialize)]
pub struct UsersWrapper {
    pub users: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

#[derive(Deserialize)]
pub struct JWTLogin<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

impl<'r> JWTLogin<'r> {
    pub async fn authenticate(
        &self,
        connection: &mut PgConnection,
        secret: &'r str,
    ) -> Result<Auth, OurError> {
        let auth_error =
            || OurError::new_bad_request_error(String::from("Cannot verify password"), None);
        let user = User::find_by_login(
            connection,
            &Login {
                username: self.username,
                password: self.password,
                authenticity_token: "",
            },
        )
        .await
        .map_err(|_| auth_error())?;
        verify_password(&Argon2::default(), &user.password_hash, self.password)?;
        let user_uuid = &user.uuid.to_string();

        let key: Hmac<Sha256> =
            Hmac::new_from_slice(secret.as_bytes()).map_err(|_| auth_error())?;
        let mut claims = BTreeMap::new();
        claims.insert("user_uuid", user_uuid);

        let token = claims.sign_with_key(&key).map_err(|_| auth_error())?;
        Ok(Auth {
            token: token.as_str().to_string(),
        })
    }
}

#[derive(Serialize)]
pub struct Auth {
    pub token: String,
}
