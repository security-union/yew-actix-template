use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub type PostgresPool = PgPool;

pub fn get_database_url() -> String {
    env::var("PG_URL").unwrap()
}

pub async fn get_pool() -> Result<PostgresPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&get_database_url())
        .await?;
    Ok(pool)
}
