use chrono::{offset::Utc, DateTime, NaiveDateTime, TimeZone};
use rocket::form::{self, DataField, FromFormField, ValueField};
use sqlx::decode::Decode;
use sqlx::error::BoxDynError;
use sqlx::postgres::{PgValueRef, Postgres};

#[derive(Debug)]
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

impl<'r> Decode<'r, Postgres> for OurDateTime {
    fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
        let naive = <NaiveDateTime as Decode<Postgres>>::decode(value)?;
        Ok(OurDateTime(Utc.from_utc_datetime(&naive)))
    }
}
