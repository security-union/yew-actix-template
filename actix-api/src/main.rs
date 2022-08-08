use actix_cors::Cors;
use actix_web::{
    cookie::{
        time::{Duration, OffsetDateTime},
        Cookie, SameSite,
    },
    get, http,
    web::{self, Json},
    App, Error, HttpResponse, HttpServer,
};

use crate::db::{get_pool, PostgresPool};
use anyhow::Result as Anysult;
use log::info;
use oauth2::{CsrfToken, PkceCodeChallenge};
use reqwest::{header::LOCATION, Client};
use serde::{Deserialize, Serialize};
use std::result;
use types::HelloResponse;

pub type Result2<T> = result::Result<T, Error>;
pub(crate) struct DecodedJwtPartClaims {
    b64_decoded: Vec<u8>,
}

pub(crate) fn b64_decode<T: AsRef<[u8]>>(input: T) -> Anysult<Vec<u8>> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD).map_err(|e| e.into())
}

impl DecodedJwtPartClaims {
    pub fn from_jwt_part_claims(encoded_jwt_part_claims: impl AsRef<[u8]>) -> Anysult<Self> {
        Ok(Self {
            b64_decoded: b64_decode(encoded_jwt_part_claims)?,
        })
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(&'a self) -> Anysult<T> {
        Ok(serde_json::from_slice(&self.b64_decoded)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

pub struct OAuthRequest {
    pkce_challenge: String,
    pkce_verifier: String,
    csrf_state: String,
}

#[derive(Deserialize)]
pub struct OAuthResponse {
    access_token: String,
    token_type: String,
    scope: String,
    id_token: String,
    refresh_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    email: String,
    name: String,
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
    let pool2 = pool.clone();
    // TODO: handle error.
    let user = web::block(move || {
        let pool = pool.clone();
        let connection = pool.get();
        let mut connection = connection.unwrap();
        let result = connection.query("SELECT * from users", &[]).unwrap();
    })
    .await
    .unwrap();

    let csrf_state = CsrfToken::new_random();
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // Store request
    let store_request = {
        let pool = pool2.clone();
        let csrf_state = csrf_state.clone();
        let pkce_challenge = pkce_challenge.clone();
        let pkce_verifier = pkce_verifier.secret().clone();
        web::block(move || {
            let connection = pool.get();
            let mut connection = connection.unwrap();
            let result = connection
                .query(
                    "INSERT INTO oauth_requests (pkce_challenge, pkce_verifier, csrf_state)
                       VALUES ($1, $2, $3)
            ",
                    &[
                        &pkce_challenge.as_str(),
                        &pkce_verifier.as_str(),
                        &csrf_state.secret().clone(),
                    ],
                )
                .unwrap();
        })
    }
    .await
    .unwrap();

    // TODO: add verify code, this needs a database.
    let google_login_url = format!("{oauth_url}?client_id={client_id}&redirect_uri={redirect_url}&response_type=code&scope={scope}&prompt=select_account&pkce_challenge={pkce_challenge}&state={state}",
                                    oauth_url=OAUTH_AUTH_URL,
                                    redirect_url=OAUTH_REDIRECT_URL,
                                    client_id=OAUTH_CLIENT_ID,
                                    scope=SCOPE,
                                    pkce_challenge=pkce_challenge.as_str(),
                                    state=&csrf_state.secret()
    );
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
    let state = info.state.clone();
    let oauth_request = {
        let pool = pool.clone();
        web::block(move || {
            let connection = pool.get();
            let mut connection = connection.unwrap();
            let result = connection
                .query(
                    "SELECT * FROM oauth_requests WHERE csrf_state=$1",
                    &[&state],
                )
                .unwrap();
            result.iter().fold(None, |acc, row| {
                Some(OAuthRequest {
                    csrf_state: row.get("csrf_state"),
                    pkce_challenge: row.get("pkce_challenge"),
                    pkce_verifier: row.get("pkce_verifier"),
                })
            })
        })
    }
    .await
    .unwrap()
    .unwrap();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", OAUTH_REDIRECT_URL),
        ("client_id", OAUTH_CLIENT_ID),
        ("code", &info.code),
        ("client_secret", OAUTH_SECRET),
        ("pkce_verifier", &oauth_request.pkce_verifier),
    ];

    let response = client
        .post(OAUTH_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .unwrap();
    let oauth_response: OAuthResponse = response.json().await.unwrap();
    let claims: Vec<&str> = oauth_response.id_token.split(".").collect();
    let decoded_claims =
        DecodedJwtPartClaims::from_jwt_part_claims(claims.get(1).unwrap()).unwrap();
    let claims: Claims = decoded_claims.deserialize().unwrap();

    // Store tokens.
    let store_tokens =
        {
            let claims = claims.clone();
            let pool = pool.clone();
            web::block(move || {
                let connection = pool.get();
                let mut connection = connection.unwrap();
                let result = connection
                .query(
                    "INSERT INTO users (email, access_token, refresh_token) VALUES ($1, $2, $3)",
                    &[&claims.email, &oauth_response.access_token, &oauth_response.refresh_token],
                )
                .unwrap();
            })
        }
        .await
        .unwrap();

    // If successful
    let cookie = Cookie::build("email", claims.email)
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
