use actix_api::get_app;
use actix_web::test;

/// Test login inserts pkce_challenge, pkce_verifier, csrf_state
/// And returns a login url with the pkce_challenge
/// 

#[actix_web::test]
async fn test_login() {
    let mut app = test::init_service(get_app()).await;
    let req = test::TestRequest::get().uri("/login").to_request();
    let resp = test::call_service(&mut app, req).await;
    drop(app);
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body = String::from_utf8(body.to_vec()).unwrap();
    assert!(body.contains("https://accounts.google.com/o/oauth2/v2/auth"));
    assert!(body.contains("code_challenge="));
    assert!(body.contains("state="));
}