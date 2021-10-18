use rocket::form::FromFormField;
use rocket_db_pools::sqlx;
use std::string::ToString;

#[derive(sqlx::Type, Debug, FromFormField)]
#[repr(i32)]
pub enum UserStatus {
    Inactive = 0,
    Active = 1,
}

impl ToString for UserStatus {
    fn to_string(&self) -> String {
        match *self {
            UserStatus::Inactive => String::from("Inactive"),
            UserStatus::Active => String::from("Active"),
        }
    }
}
