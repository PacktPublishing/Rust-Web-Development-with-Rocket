use super::HtmlResponse;
use crate::fairings::db::DBConnection;
use crate::models::{
    pagination::Pagination,
    post::{Post, ShowPost},
    user::User,
};
use rocket::form::Form;
use rocket::http::Status;
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

    let context = context! { user, post: &(post.to_show_post())};
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

    let show_posts: Vec<ShowPost> = posts.into_iter().map(|post| post.to_show_post()).collect();
    let context =
        context! {user, posts: &show_posts, pagination: new_pagination.map(|pg|pg.to_context())};
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
