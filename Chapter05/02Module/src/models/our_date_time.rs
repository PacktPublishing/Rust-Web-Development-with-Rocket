use chrono::{offset::Utc, DateTime};
use rocket::form::{self, DataField, FromFormField, ValueField};

#[derive(Debug)]
pub struct OurDateTime(DateTime<Utc>);
#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(_: ValueField<'r>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }

    async fn from_data(_: DataField<'r, '_>) -> form::Result<'r, Self> {
        todo!("will implement later")
    }
}
