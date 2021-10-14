use rocket::form::FromFormField;
use rocket_db_pools::sqlx;

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
}
