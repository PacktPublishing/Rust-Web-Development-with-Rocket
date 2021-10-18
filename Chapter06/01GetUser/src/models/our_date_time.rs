use chrono::{offset::Utc, DateTime};
use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct OurDateTime(pub DateTime<Utc>);
#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }

    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}
