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

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin(format!("http://localhost:{}", UI_PORT).as_str())
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
