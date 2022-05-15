use super::HtmlResponse;
use crate::errors::our_error::OurError;
use crate::fairings::csrf::Token as CsrfToken;
use crate::fairings::db::DBConnection;
use crate::guards::auth::CurrentUser;
use crate::models::{
    pagination::Pagination,
    post::{NewPost, Post, ShowPost},
    post_type::PostType,
    user::User,
    worker::Message,
};
use flume::Sender;
use image::codecs::jpeg::JpegEncoder;
use image::error::ImageError;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageEncoder};
use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};
use std::io::Cursor;
use std::ops::Deref;
use std::path::Path;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncReadExt;

#[get("/users/<user_uuid>/posts/<uuid>", format = "text/html")]
pub async fn get_post(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
    current_user: Option<CurrentUser>,
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

    let context = context! {
        user,
        current_user,
        post: &(post.to_show_post()),
    };
    Ok(Template::render("posts/show", context))
}

#[get("/users/<user_uuid>/posts?<pagination>", format = "text/html")]
pub async fn get_posts(
    mut db: Connection<DBConnection>,
    flash: Option<FlashMessage<'_>>,
    user_uuid: &str,
    pagination: Option<Pagination>,
    csrf_token: CsrfToken,
    current_user: Option<CurrentUser>,
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
        current_user,
        posts: &show_posts,
        pagination: new_pagination.map(|pg|pg.to_context()),
        csrf_token,
    };
    Ok(Template::render("posts/index", context))
}

#[post(
    "/users/<user_uuid>/posts",
    format = "multipart/form-data",
    data = "<upload>",
    rank = 1
)]
pub async fn create_post<'r>(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    mut upload: Form<NewPost<'r>>,
    tx: &State<Sender<Message>>,
    csrf_token: CsrfToken,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let create_err = || {
        Flash::error(
            Redirect::to(format!("/users/{}/posts", user_uuid)),
            "Something went wrong when uploading file",
        )
    };
    csrf_token
        .verify(&upload.authenticity_token)
        .map_err(|_| create_err())?;
    if current_user.is_not(user_uuid) {
        return Err(create_err());
    }
    let file_uuid = uuid::Uuid::new_v4().to_string();
    if upload.file.content_type().is_none() {
        return Err(create_err());
    }
    let ext = upload.file.content_type().unwrap().extension().unwrap();
    let tmp_filename = format!("/tmp/{}.{}", &file_uuid, &ext);
    upload
        .file
        .persist_to(tmp_filename)
        .await
        .map_err(|_| create_err())?;
    let mut content = String::new();
    let mut post_type = PostType::Text;
    let mt = upload.file.content_type().unwrap().deref();
    let mut wm = Message::new();
    let mut is_video = false;
    if mt.is_text() {
        let orig_path = upload.file.path().unwrap().to_string_lossy().to_string();
        let mut text_content = vec![];
        let _ = File::open(orig_path)
            .await
            .map_err(|_| create_err())?
            .read_to_end(&mut text_content)
            .await
            .map_err(|_| create_err())?;
        content.push_str(std::str::from_utf8(&text_content).unwrap());
    } else if mt.is_bmp() || mt.is_jpeg() || mt.is_png() || mt.is_gif() {
        post_type = PostType::Photo;
        let orig_path = upload.file.path().unwrap().to_string_lossy().to_string();
        let dest_filename = format!("{}.jpg", file_uuid);
        content.push_str("/assets/");
        content.push_str(&dest_filename);

        let orig_file = tokio::fs::read(orig_path).await.map_err(|_| create_err())?;
        let read_buffer = Cursor::new(orig_file);
        let encoded_result: Result<DynamicImage, ()> = tokio::task::spawn_blocking(|| {
            Ok(ImageReader::new(read_buffer)
                .with_guessed_format()
                .map_err(|_| ())?
                .decode()
                .map_err(|_| ())?)
        })
        .await
        .map_err(|_| create_err())?;
        let image = encoded_result.map_err(|_| create_err())?;

        let write_result: Result<Vec<u8>, ImageError> = tokio::task::spawn_blocking(move || {
            let mut write_buffer: Vec<u8> = vec![];
            let mut write_cursor = Cursor::new(&mut write_buffer);
            let _ = JpegEncoder::new_with_quality(&mut write_cursor, 75).write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color(),
            )?;

            Ok(write_buffer)
        })
        .await
        .map_err(|_| create_err())?;
        let write_bytes = write_result.map_err(|_| create_err())?;
        let dest_path = Path::new(rocket::fs::relative!("static")).join(&dest_filename);
        tokio::fs::write(dest_path, &write_bytes)
            .await
            .map_err(|_| create_err())?;
    } else if mt.is_svg() {
        post_type = PostType::Photo;
        let dest_filename = format!("{}.svg", file_uuid);
        content.push_str("/assets/");
        content.push_str(&dest_filename);
        let dest_path = Path::new(rocket::fs::relative!("static")).join(&dest_filename);
        upload
            .file
            .move_copy_to(&dest_path)
            .await
            .map_err(|_| create_err())?;
    } else if mt.is_mp4() || mt.is_mpeg() || mt.is_ogg() || mt.is_mov() || mt.is_webm() {
        post_type = PostType::Video;
        let dest_filename = format!("{}.mp4", file_uuid);
        content.push_str("loading/assets/");
        content.push_str(&dest_filename);
        is_video = true;
        wm.orig_filename = upload
            .file
            .path()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .clone();
        wm.dest_filename = dest_filename.clone();
    } else {
        return Err(create_err());
    }
    let connection = db.acquire().await.map_err(|_| create_err())?;

    Ok(Post::create(connection, user_uuid, post_type, &content)
        .await
        .and_then(move |post| {
            if is_video {
                wm.uuid = post.uuid.to_string();
                let _ = tx.send(wm).map_err(|_| {
                    OurError::new_internal_server_error(
                        String::from("Cannot process message"),
                        None,
                    )
                })?;
            }
            Ok(Flash::success(
                Redirect::to(format!("/users/{}/posts", user_uuid)),
                "Successfully created post",
            ))
        })
        .map_err(|_| create_err())?)
}

#[post(
    "/users/<user_uuid>/posts/delete/<uuid>",
    format = "application/x-www-form-urlencoded"
)]
pub async fn delete_post(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    uuid: &str,
    current_user: CurrentUser,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let delete_err = || {
        Flash::error(
            Redirect::to(format!("/users/{}/posts", user_uuid)),
            "Something went wrong when deleting post",
        )
    };
    if current_user.is_not(user_uuid) {
        return Err(delete_err());
    }
    let connection = db.acquire().await.map_err(|_| delete_err())?;
    let post = Post::find(connection, uuid)
        .await
        .map_err(|_| delete_err())?;
    if post.user_uuid.to_string() != user_uuid {
        return Err(delete_err());
    }

    Ok({
        let _ = Post::destroy(connection, uuid)
            .await
            .map_err(|_| delete_err())?;
        if post.post_type == PostType::Photo || post.post_type == PostType::Video {
            remove_file(post.content.replacen("/assets/", "static/", 1))
                .await
                .map_err(|_| delete_err())?;
        }

        Flash::success(
            Redirect::to(format!("/users/{}/posts", user_uuid)),
            "Successfully deleted post",
        )
    })
}
