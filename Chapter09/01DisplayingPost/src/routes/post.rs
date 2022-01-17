use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{pagination::Pagination, post::Post, post_type::PostType, user::User};
use crate::traits::DisplayPostContent;
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::Serialize;
use rocket_db_pools::{sqlx::Acquire, Connection};
use rocket_dyn_templates::{context, Template};

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

    #[derive(Serialize)]
    struct ShowPost {
        post_html: String,
    }
    #[derive(Serialize)]
    struct Context {
        user: User,
        post: ShowPost,
    }

    let mut post_html = String::new();
    match post.post_type {
        PostType::Text => post_html = post.to_text().raw_html(),
        PostType::Photo => post_html = post.to_photo().raw_html(),
        PostType::Video => post_html = post.to_video().raw_html(),
    }

    let context = Context {
        user,
        post: ShowPost { post_html },
    };
    Ok(Template::render("posts/show", context))
}

#[get("/users/<user_uuid>/posts?<pagination>", format = "text/html")]
pub async fn get_posts(
    mut db: Connection<DBConnection>,
    user_uuid: &str,
    pagination: Option<Pagination>,
) -> HtmlResponse {
    let user = User::find(&mut db, user_uuid).await.map_err(|e| e.status)?;
    let (posts, new_pagination) = Post::find_all(&mut db, user_uuid, pagination)
        .await
        .map_err(|e| e.status)?;

    #[derive(Serialize)]
    struct ShowPost {
        uuid: String,
        post_html: String,
    }

    let show_posts: Vec<ShowPost> = posts
        .into_iter()
        .map(|post| {
            let uuid = post.uuid.to_string();
            let mut post_html = String::new();
            match post.post_type {
                PostType::Text => post_html = post.to_text().raw_html(),
                PostType::Photo => post_html = post.to_photo().raw_html(),
                PostType::Video => post_html = post.to_video().raw_html(),
            };
            ShowPost { uuid, post_html }
        })
        .collect();
    let context =
        context! {user, posts: show_posts, pagination: new_pagination.map(|pg|pg.to_context())};
    Ok(Template::render("posts/index", context))
}

#[post("/users/<_user_uuid>/posts", format = "text/html", data = "<_upload>")]
pub async fn create_post(
    mut _db: Connection<DBConnection>,
    _user_uuid: &str,
    _upload: Form<Post>,
) -> HtmlResponse {
    todo!("will implement later")
}

#[delete("/users/<_user_uuid>/posts/<_uuid>", format = "text/html")]
pub async fn delete_post(
    mut _db: Connection<DBConnection>,
    _user_uuid: &str,
    _uuid: &str,
) -> HtmlResponse {
    todo!("will implement later")
}
