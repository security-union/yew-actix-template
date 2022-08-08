use actix_cors::Cors;
use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie, SameSite,
    },
    get, http,
    web::{self, Json},
    App, Error, HttpResponse, HttpServer, Responder,
};
use log::info;
use reqwest::{header::LOCATION, Client};
use serde::Deserialize;
use types::HelloResponse;

use crate::db::{get_pool, PostgresPool};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    state: Option<String>,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

pub mod db;

const OAUTH_CLIENT_ID: &str = std::env!("OAUTH_CLIENT_ID");
const OAUTH_AUTH_URL: &str = std::env!("OAUTH_AUTH_URL");
const OAUTH_TOKEN_URL: &str = std::env!("OAUTH_TOKEN_URL");
const OAUTH_SECRET: &str = std::env!("OAUTH_CLIENT_SECRET");
const OAUTH_REDIRECT_URL: &str = std::env!("OAUTH_REDIRECT_URL");
const SCOPE: &str = "email%20profile%20openid";
const AFTER_LOGIN_URL: &str = "http://localhost/";

#[get("/login")]
async fn login(pool: web::Data<PostgresPool>) -> Result<HttpResponse, Error> {
    // TODO: verify if user exists in the db by looking at the session cookie, (if the client provides one.)

    // TODO: handle error.
    let user = web::block(move || {
        let connection = pool.get();
        let mut connection = connection.unwrap();
        let result = connection.query("SELECT * from users", &[]).unwrap();
        info!("result {:?}", result);
    })
    .await
    .unwrap();

    // TODO: add verify code, this needs a database.
    let google_login_url = format!("{oauth_url}?client_id={client_id}&redirect_uri={redirect_url}&response_type=code&scope={scope}&prompt=select_account",
                                    oauth_url=OAUTH_AUTH_URL,
                                    redirect_url=OAUTH_REDIRECT_URL,
                                    client_id=OAUTH_CLIENT_ID,
                                    scope=SCOPE
    );
    info!("url {}", google_login_url);
    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, google_login_url));
    Ok(response.body(""))
}

#[get("/login/callback")]
async fn handle_google_oauth_callback(
    pool: web::Data<PostgresPool>,
    info: web::Query<AuthRequest>,
) -> HttpResponse {
    info!("info {:?}", info);
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", OAUTH_REDIRECT_URL),
        ("client_id", OAUTH_CLIENT_ID),
        ("code", &info.code),
        ("client_secret", OAUTH_SECRET),
    ];

    let res = client.post(OAUTH_TOKEN_URL).form(&params).send().await;

    // Access token.
    // TODO: save tokens, and user email to a database.
    info!("response {:?}", &res);
    info!("body {:?}", res.unwrap().text().await);

    // If successful
    let cookie = Cookie::build("name", "value")
        .path("/")
        .same_site(SameSite::Lax)
        // Session lasts only 360 secs to test cookie expiration.
        .expires(OffsetDateTime::now_utc().checked_add(Duration::seconds(360)))
        .finish();

    let mut response = HttpResponse::Found();
    response.append_header((LOCATION, AFTER_LOGIN_URL));
    response.cookie(cookie);
    response.body("login success")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        name: name.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
    const UI_PORT: &str = std::env!("TRUNK_SERVE_PORT");
    const UI_HOST: &str = std::env!("TRUNK_SERVE_HOST");

    // TODO: Deal with https, maybe we should just expose this as an env var?
    let allowed_origin = if UI_PORT != "80" {
        format!("http://{}:{}", UI_HOST, UI_PORT)
    } else {
        format!("http://{}", UI_HOST)
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(allowed_origin.as_str())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let pool = get_pool();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(greet)
            .service(handle_google_oauth_callback)
            .service(login)
    })
    .bind(("0.0.0.0", ACTIX_PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}
