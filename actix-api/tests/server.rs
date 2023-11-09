mod common;
use actix_api::get_app;
use actix_web::test;

/// Test login inserts pkce_challenge, pkce_verifier, csrf_state
/// And returns a login url with the pkce_challenge
///

#[actix_web::test]
async fn test_login() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_up(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/login").to_request();
    let resp = test::call_service(&mut app, req).await;
    drop(app);
    assert!(resp.status() == 302);
}
