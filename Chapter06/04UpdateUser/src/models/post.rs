use super::our_date_time::OurDateTime;
use super::post_type::PostType;
use rocket::form::FromForm;
use rocket::fs::TempFile;
use uuid::Uuid;

#[derive(FromForm)]
pub struct Post<'r> {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub post_type: PostType,
    pub content: String,
    pub upload_data: TempFile<'r>,
    pub created_at: OurDateTime,
}
