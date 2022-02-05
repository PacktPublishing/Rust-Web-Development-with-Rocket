use rocket::form::FromFormField;
use rocket_db_pools::sqlx;

#[derive(Copy, Clone, sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
pub enum PostType {
    Text = 0,
    Photo = 1,
    Video = 2,
}
