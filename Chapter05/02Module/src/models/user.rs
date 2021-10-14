use super::our_date_time::OurDateTime;
use super::user_status::UserStatus;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, FromForm)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub description: String,
    pub status: UserStatus,
    pub created_at: OurDateTime,
    pub updated_at: OurDateTime,
}
