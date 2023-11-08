use actix_api::{get_app, ACTIX_PORT};
use actix_cors::Cors;
use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie, SameSite,
    },
    error, get, http,
    web::{self, Json},
    App, Error, HttpResponse, HttpServer,
};

use actix_api::auth::{
    fetch_oauth_request, generate_and_store_oauth_request, request_token, upsert_user,
};
use actix_api::{
    auth::AuthRequest,
    db::{get_pool, PostgresPool},
};
use reqwest::header::LOCATION;
use types::HelloResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(move || {
        get_app()
    }).bind(("0.0.0.0", ACTIX_PORT.parse::<u16>().unwrap()))?.run().await
}
