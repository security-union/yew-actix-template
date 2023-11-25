mod common;
use actix_api::get_app;
use actix_web::test;

use types::HelloResponse;
/// Test login inserts pkce_challenge, pkce_verifier, csrf_state
/// And returns a login url with the pkce_challenge
///

#[actix_web::test]
async fn test_login() {
    let db_url = std::env::var("PG_URL").unwrap();
    println!("DB_URL: {}", db_url);
    common::dbmate_rebuild(&db_url);
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/hello/dario").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    // Get body as json
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    let body: HelloResponse = serde_json::from_str(&body).unwrap();
    assert_eq!(
        body,
        HelloResponse {
            name: "dario".to_string()
        }
    );
}
