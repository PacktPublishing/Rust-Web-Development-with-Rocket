use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::guards::RawContentType;
use crate::models::{
    pagination::Pagination,
    post::{Post, ShowPost},
    post_type::PostType,
    user::User,
};
use multer::Multipart;
use rocket::data::{ByteUnit, Data};
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};

const TEXT_LIMIT: ByteUnit = ByteUnit::Kibibyte(64);

#[get("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
pub async fn get_post(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
) -> HtmlResponse {
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let user = User::find(connection, user_uuid)
        .await
        .map_err(|e| e.status)?;
    let connection = db
        .acquire()
        .await
        .map_err(|_| Status::InternalServerError)?;
    let post = Post::find(connection, uuid).await.map_err(|e| e.status)?;
    if post.user_uuid != user.uuid {
        return Err(Status::InternalServerError);
    }

    let context = context! { user, post: &(post.to_show_post())};
    Ok(Template::render("posts/show", context))
}

#[get("/users/<user_uuid>/posts?<pagination>", format = "text/html")]
pub async fn get_posts(
    mut db: Connection<DBConnection>,
    flash: Option<FlashMessage<'_>>,
    user_uuid: &str,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    let flash_message = flash.map(|fm| String::from(fm.message()));
    let user = User::find(&mut db, user_uuid).await.map_err(|e| e.status)?;
    let (posts, new_pagination) = Post::find_all(&mut db, user_uuid, pagination)
        .await
        .map_err(|e| e.status)?;

    let show_posts: Vec<ShowPost> = posts.into_iter().map(|post| post.to_show_post()).collect();
    let context = context! {
    flash: flash_message,
    user,
    posts: &show_posts,
    pagination: new_pagination.map(|pg|pg.to_context())};
    Ok(Template::render("posts/index", context))
}

#[post(
    "/users/<user_uuid>/posts",
    format = "multipart/form-data",
    data = "<upload>",
    rank = 1
)]
pub async fn create_post(
    mut db: Connection<DBConnection>,
    content_type: RawContentType<'_>,
    user_uuid: &str,
    upload: Data<'_>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let create_err = || {
        Flash::error(
            Redirect::to(format!("/users/{}/posts", user_uuid)),
            "Something went wrong when uploading file",
        )
    };
    let boundary = multer::parse_boundary(content_type.0).map_err(|_| create_err())?;
    let upload_stream = upload.open(TEXT_LIMIT);
    let mut multipart = Multipart::new(tokio_util::io::ReaderStream::new(upload_stream), boundary);
    let mut text_post = String::new();
    while let Some(mut field) = multipart.next_field().await.map_err(|_| create_err())? {
        let field_name = field.name();
        let file_name = field.file_name();
        let content_type = field.content_type();
        println!(
            "Field name: {:?}, File name: {:?}, Content-Type: {:?}",
            field_name, file_name, content_type
        );
        while let Some(field_chunk) = field.chunk().await.map_err(|_| create_err())? {
            text_post.push_str(std::str::from_utf8(field_chunk.as_ref()).unwrap());
        }
    }
    let connection = db.acquire().await.map_err(|_| create_err())?;
    Post::create(connection, user_uuid, PostType::Text, &text_post)
        .await
        .map_err(|_| create_err())?;
    Ok(Flash::success(
        Redirect::to(format!("/users/{}/posts", user_uuid)),
        "Successfully created post",
    ))
}

#[delete("/users/<_user_uuid>/posts/<_uuid>", format = "text/html")]
pub async fn delete_post(
    mut _db: Connection<DBConnection>,
    _user_uuid: &str,
    _uuid: &str,
) -> HtmlResponse {
    todo!("will implement later")
}
