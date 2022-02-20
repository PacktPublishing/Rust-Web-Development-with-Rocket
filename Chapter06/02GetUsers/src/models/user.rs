use super::bool_wrapper::BoolWrapper;
use super::our_date_time::OurDateTime;
use super::pagination::{Pagination, DEFAULT_LIMIT};
use super::user_status::UserStatus;
use crate::fairings::db::DBConnection;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::{Acquire, FromRow, PgConnection};
use rocket_db_pools::Connection;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, FromRow, FromForm)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub description: Option<String>,
    pub status: UserStatus,
    pub created_at: OurDateTime,
    pub updated_at: OurDateTime,
}

impl User {
    pub async fn find(connection: &mut PgConnection, uuid: &str) -> Result<Self, Box<dyn Error>> {
        let parsed_uuid = Uuid::parse_str(uuid)?;
        let query_str = "SELECT * FROM users WHERE uuid = $1";
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await?)
    }

    pub async fn find_all(
        db: &mut Connection<DBConnection>,
        pagination: Option<Pagination>,
    ) -> Result<(Vec<Self>, Option<Pagination>), Box<dyn Error>> {
        if pagination.is_some() {
            return Self::find_all_with_pagination(db, &(pagination.unwrap())).await;
        } else {
            return Self::find_all_without_pagination(db).await;
        }
    }

    async fn find_all_without_pagination(
        db: &mut Connection<DBConnection>,
    ) -> Result<(Vec<Self>, Option<Pagination>), Box<dyn Error>> {
        let query_str = "SELECT * FROM users ORDER BY created_at DESC LIMIT $1";
        let connection = db.acquire().await?;
        let users = sqlx::query_as::<_, Self>(query_str)
            .bind(DEFAULT_LIMIT as i32)
            .fetch_all(connection)
            .await?;
        let mut new_pagination: Option<Pagination> = None;
        if users.len() == DEFAULT_LIMIT {
            let query_str = "SELECT EXISTS(SELECT 1 FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&users.last().unwrap().created_at)
                .fetch_one(connection)
                .await?;
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
    ) -> Result<(Vec<Self>, Option<Pagination>), Box<dyn Error>> {
        let query_str =
            "SELECT * FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT $2";
        let connection = db.acquire().await?;
        let users = sqlx::query_as::<_, Self>(query_str)
            .bind(&pagination.next)
            .bind(pagination.limit as i32)
            .fetch_all(connection)
            .await?;
        let mut new_pagination: Option<Pagination> = None;
        if users.len() == pagination.limit {
            let query_str = "SELECT EXISTS(SELECT 1 FROM users WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&users.last().unwrap().created_at)
                .fetch_one(connection)
                .await?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: users.last().unwrap().created_at.to_owned(),
                    limit: pagination.limit,
                });
            }
        }
        Ok((users, new_pagination))
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
