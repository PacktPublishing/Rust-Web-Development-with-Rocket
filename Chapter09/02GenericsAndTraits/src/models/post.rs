use super::bool_wrapper::BoolWrapper;
use super::our_date_time::OurDateTime;
use super::pagination::{Pagination, DEFAULT_LIMIT};
use super::photo_post::PhotoPost;
use super::post_type::PostType;
use super::text_post::TextPost;
use super::video_post::VideoPost;
use crate::errors::our_error::OurError;
use crate::fairings::db::DBConnection;
use crate::traits::DisplayPostContent;
use rocket::form::FromForm;
use rocket_db_pools::sqlx::{FromRow, PgConnection};
use rocket_db_pools::{sqlx::Acquire, Connection};
use uuid::Uuid;

#[derive(FromRow, FromForm)]
pub struct Post {
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub post_type: PostType,
    pub content: String,
    pub created_at: OurDateTime,
}

impl Post {
    pub fn to_text(self) -> TextPost {
        TextPost(self)
    }

    pub fn to_photo(self) -> PhotoPost {
        PhotoPost(self)
    }

    pub fn to_video(self) -> VideoPost {
        VideoPost(self)
    }

    pub fn to_media(self) -> Box<dyn DisplayPostContent> {
        match self.post_type {
            PostType::Text => Box::new(self.to_text()),
            PostType::Photo => Box::new(self.to_photo()),
            PostType::Video => Box::new(self.to_video()),
        }
    }

    pub async fn find(connection: &mut PgConnection, uuid: &str) -> Result<Post, OurError> {
        let parsed_uuid = Uuid::parse_str(uuid).map_err(OurError::from_uuid_error)?;
        let query_str = "SELECT * FROM posts WHERE uuid = $1";
        Ok(sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .fetch_one(connection)
            .await
            .map_err(OurError::from_sqlx_error)?)
    }

    pub async fn find_all(
        db: &mut Connection<DBConnection>,
        user_uuid: &str,
        pagination: Option<Pagination>,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        if pagination.is_some() {
            return Self::find_all_with_pagination(db, user_uuid, &pagination.unwrap()).await;
        } else {
            return Self::find_all_without_pagination(db, user_uuid).await;
        }
    }

    async fn find_all_without_pagination(
        db: &mut Connection<DBConnection>,
        user_uuid: &str,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        let parsed_uuid = Uuid::parse_str(user_uuid).map_err(OurError::from_uuid_error)?;
        let query_str = r#"SELECT *
FROM posts
WHERE user_uuid = $1
ORDER BY created_at DESC
LIMIT $2"#;
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let posts = sqlx::query_as::<_, Self>(query_str)
            .bind(parsed_uuid)
            .bind(DEFAULT_LIMIT as i32)
            .fetch_all(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        let mut new_pagination: Option<Pagination> = None;
        if posts.len() == DEFAULT_LIMIT {
            let query_str = "SELECT EXISTS(SELECT 1 FROM posts WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&posts.last().unwrap().created_at)
                .fetch_one(connection)
                .await
                .map_err(OurError::from_sqlx_error)?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: posts.last().unwrap().created_at.to_owned(),
                    limit: DEFAULT_LIMIT,
                });
            }
        }
        Ok((posts, new_pagination))
    }

    async fn find_all_with_pagination(
        db: &mut Connection<DBConnection>,
        user_uuid: &str,
        pagination: &Pagination,
    ) -> Result<(Vec<Self>, Option<Pagination>), OurError> {
        let parsed_uuid = Uuid::parse_str(user_uuid).map_err(OurError::from_uuid_error)?;
        let query_str = r#"SELECT *
FROM posts
WHERE user_uuid = $1 AND　created_at < $2
ORDER BY created_at　DESC
LIMIT $3"#;
        let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
        let posts = sqlx::query_as::<_, Self>(query_str)
            .bind(&parsed_uuid)
            .bind(&pagination.next)
            .bind(pagination.limit as i32)
            .fetch_all(connection)
            .await
            .map_err(OurError::from_sqlx_error)?;
        let mut new_pagination: Option<Pagination> = None;
        if posts.len() == pagination.limit {
            let query_str = "SELECT EXISTS(SELECT 1 FROM posts WHERE created_at < $1 ORDER BY created_at DESC LIMIT 1)";
            let connection = db.acquire().await.map_err(OurError::from_sqlx_error)?;
            let exists = sqlx::query_as::<_, BoolWrapper>(query_str)
                .bind(&posts.last().unwrap().created_at)
                .fetch_one(connection)
                .await
                .map_err(OurError::from_sqlx_error)?;
            if exists.0 {
                new_pagination = Some(Pagination {
                    next: posts.last().unwrap().created_at.to_owned(),
                    limit: pagination.limit,
                });
            }
        }
        Ok((posts, new_pagination))
    }
}
