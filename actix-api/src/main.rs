use actix_api::{get_app, ACTIX_PORT};
use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(get_app)
        .bind(("0.0.0.0", ACTIX_PORT.parse::<u16>().unwrap()))?
        .run()
        .await
}
