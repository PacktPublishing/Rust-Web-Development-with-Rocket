use rocket_db_pools::sqlx::FromRow;

#[derive(FromRow)]
pub struct BoolWrapper(pub bool);
