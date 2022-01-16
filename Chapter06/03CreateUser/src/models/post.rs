use super::our_date_time::OurDateTime;
use super::post_type::PostType;
use rocket::form::FromForm;
use uuid::Uuid;

#[derive(FromForm)]
pub struct Post {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub post_type: PostType,
    pub content: String,
    pub created_at: OurDateTime,
}
