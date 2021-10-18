use super::our_date_time::OurDateTime;
use super::pagination::Pagination;
use super::user_status::UserStatus;
use crate::fairings::db::DBConnection;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::FromRow;
use rocket_db_pools::Connection;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, FromRow, FromForm)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub description: Option<String>,
    pub status: UserStatus,
    pub created_at: OurDateTime,
    pub updated_at: OurDateTime,
}

impl User {
    pub async fn find(
        mut db: Connection<DBConnection>,
        uuid: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let parsed_uuid = Uuid::parse_str(uuid)?;
        let query_str = format!("SELECT * FROM users WHERE uuid = $1");
        Ok(sqlx::query_as::<_, Self>(&query_str)
            .bind(parsed_uuid)
            .fetch_one(&mut *db)
            .await?)
    }

    pub async fn find_all(
        mut db: Connection<DBConnection>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut query_str = String::from("SELECT * FROM users");
        if pagination.is_some() {
            query_str.push_str(" WHERE created_at < $1");
        }
        query_str.push_str(" ORDER BY created_at DESC");
        if pagination.is_some() {
            query_str.push_str(&format!(" LIMIT {}", pagination.unwrap().limit));
        }
        let query = sqlx::query_as::<_, Self>(&query_str);
        Ok(query.fetch_all(&mut *db).await?)
    }

    pub fn to_html_string(&self) -> String {
        format!(
            r#"
<div><span class="label">UUID: </span>{uuid}</div>
<div><span class="label">Username: </span>{username}</div>
<div><span class="label">Email: </span>{email}</div>
<div><span class="label">Description: </span>{description}</div>
<div><span class="label">Status: </span>{status}</div>
<div><span class="label">Created At: </span>{created_at}</div>
<div><span class="label">Updated At: </span>{updated_at}</div>
"#,
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
