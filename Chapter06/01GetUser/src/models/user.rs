use super::our_date_time::OurDateTime;
use super::user_status::UserStatus;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::{FromRow, PgConnection};
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
