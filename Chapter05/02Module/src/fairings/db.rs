use rocket_db_pools::{sqlx::PgPool, Database};

#[derive(Database)]
#[database("main_connection")]
pub struct DBConnection(PgPool);
