use crate::errors::our_error::OurError;
use crate::fairings::db::DBConnection;
use crate::models::{
    pagination::Pagination,
    user::{User, UsersWrapper},
};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[get("/users", format = "json", data = "<pagination>")]
pub async fn users(
    mut db: Connection<DBConnection>,
    pagination: Option<Json<Pagination>>,
) -> Result<Json<UsersWrapper>, Json<OurError>> {
    let parsed_pagination = pagination.map(|p| p.into_inner());
    let (users, new_pagination) = User::find_all(&mut db, parsed_pagination)
        .await
        .map_err(|_| OurError::new_internal_server_error(String::from("Internal Error"), None))?;
    Ok(Json(UsersWrapper {
        users,
        pagination: new_pagination,
    }))
}
