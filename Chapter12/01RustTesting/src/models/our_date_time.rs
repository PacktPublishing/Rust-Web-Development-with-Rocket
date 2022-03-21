use chrono::{offset::Utc, DateTime, TimeZone};
use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField, ValueField};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Clone, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct OurDateTime(pub DateTime<Utc>);

#[rocket::async_trait]
impl<'r> FromFormField<'r> for OurDateTime {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let timestamp = field.value.parse::<i64>()?;
        Ok(OurDateTime(Utc.timestamp_nanos(timestamp)))
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let limit = field
            .request
            .limits()
            .get("form")
            .unwrap_or_else(|| 8.kibibytes());
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            return Err((None, Some(limit)).into());
        }
        let bytes = bytes.into_inner();
        let time_string = std::str::from_utf8(&bytes)?;
        let timestamp = time_string.parse::<i64>()?;
        Ok(OurDateTime(Utc.timestamp_nanos(timestamp)))
    }
}
