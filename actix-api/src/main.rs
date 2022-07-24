use actix_cors::Cors;
use actix_web::{
    get, http,
    web::{self, Json},
    App, HttpServer,
};
use types::HelloResponse;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Json<HelloResponse> {
    Json(HelloResponse {
        name: name.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

        App::new().wrap(cors).service(greet)
    })
    .bind(("0.0.0.0", ACTIX_PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}
