use postgres::NoTls;
use r2d2::PooledConnection;
use postgres::{error::Error, IsolationLevel, NoTls, Transaction};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::env;

pub type PostgresPool = Pool<PostgresConnectionManager<NoTls>>;
pub type PostgresConnection = PooledConnection<PostgresConnectionManager<NoTls>>;

pub fn get_database_url() -> String {
    if let Ok(url) = env::var("PG_URL") {
        url
    } else {
        let db_user = env::var("PG_USER").expect("PG_USER must be set");
        let db_pass = env::var("PG_PASS").expect("PG_PASS must be set");
        let db_host = env::var("PG_HOST").expect("PG_HOST must be set");
        let db_port = env::var("PG_PORT").expect("PG_PORT must be set");
        let db_name = env::var("PG_DB").expect("PG_DB must be set");

        format!(
            "user={} password={} host={} port={} dbname={}",
            db_user, db_pass, db_host, db_port, db_name
        )
    }
}

pub fn get_pool() -> PostgresPool {
    let manager = PostgresConnectionManager::new(
        get_database_url()
            .parse()
            .expect("Database url is in a bad format."),
        NoTls,
    );
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to build a database connection pool")
}
