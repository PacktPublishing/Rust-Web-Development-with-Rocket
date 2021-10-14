use chrono::{offset::Utc, DateTime};
use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket_db_pools::sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct OurDateTime(DateTime<Utc>);
#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}
