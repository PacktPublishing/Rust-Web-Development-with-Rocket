use rocket_db_pools::{sqlx::PgPool, Connection, Database};

#[derive(Database)]
#[database("main_connection")]
pub struct DBConnection(PgPool);
