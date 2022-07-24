mod components;

use std::collections::HashMap;

use reqwasm::http::Request;
use types::HelloResponse;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*; // use `openid::*` when using OpenID connect
#[macro_use]
extern crate lazy_static;
use gloo_console::log;

use crate::components::ViewAuthInfo;

// This is read at compile time, please restart if you change this value.
const ACTIX_PORT: &str = std::env!("ACTIX_PORT");
const OAUTH_CLIENT_ID: &str = std::env!("OAUTH_CLIENT_ID");
const OAUTH_AUTH_URL: &str = std::env!("OAUTH_AUTH_URL");
const OAUTH_TOKEN_URL: &str = std::env!("OAUTH_TOKEN_URL");

fn truthy(s: String) -> bool {
    ["true".to_string(), "1".to_string()].contains(&s.to_lowercase())
}

// We need a lazy static block because these vars need to call a
// few functions.
lazy_static! {
    static ref ENABLE_OAUTH: bool = truthy(std::env!("ENABLE_OAUTH").to_string());
    static ref OAUTH_CLIENT_SECRET: Option<String> = {
        let secret = std::env!("OAUTH_CLIENT_SECRET");
        if secret != "" {
            Some(secret.to_string())
        } else {
            None
        }
    };
}

#[function_component(App)]
fn app_component() -> Html {
    log!("OAuth enabled: {}", *ENABLE_OAUTH);
    if *ENABLE_OAUTH {
        

        let login = Callback::from(|_: MouseEvent| {
            let mut query: HashMap<String,String> = HashMap::new();
            query.insert("access_type".into(), "offline".into());
            OAuth2Dispatcher::<Client>::new().start_login_opts(LoginOptions {
                query,
            });
        });
        let logout = Callback::from(|_: MouseEvent| {
            OAuth2Dispatcher::<Client>::new().logout();
        });

        let config = Config {
            client_id: OAUTH_CLIENT_ID.to_string(),
            auth_url: OAUTH_AUTH_URL.to_string(),
            token_url: OAUTH_TOKEN_URL.to_string(),
            client_secret: OAUTH_CLIENT_SECRET.clone(),
        };

        // These scopes are specific to Gmail, please modify as needed.
        let scopes: Vec<String> = vec![
            "https://www.googleapis.com/auth/gmail.readonly",
            "https://www.googleapis.com/auth/gmail.labels",
            "https://www.googleapis.com/auth/gmail.modify",
            "https://www.googleapis.com/auth/userinfo.email",
            "https://www.googleapis.com/auth/userinfo.profile"
        ]
        .iter()
        .map(|scope| scope.to_string())
        .collect();

        html! {
            <OAuth2 {config} scopes={scopes}>
                <Failure><FailureMessage/></Failure>
                <Authenticated>
                    <HttpGetExample/>
                    <ViewAuthInfo/>
                    <p> <button onclick={logout}>{ "Logout" }</button> </p>
                </Authenticated>
                <NotAuthenticated>
                    <>
                        <input type="image" onclick={login.clone()} src="/assets/btn_google.png" />
                    </>
                </NotAuthenticated>
            </OAuth2>
        }
    } else {
        html! {
            <HttpGetExample/>
        }
    }
}

#[function_component(HttpGetExample)]
fn get_example() -> Html {
    let actix_url: String = format!("http://localhost:{}", ACTIX_PORT);
    let hello_response = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let endpoint = Box::new(format!(
        "{actix_url}/hello/{name}",
        actix_url = actix_url,
        name = "world",
    ));
    let retry = {
        let hello_response = hello_response.clone();
        let error = error.clone();
        let endpoint = endpoint.clone();
        Callback::from(move |_| {
            let hello_response = hello_response.clone();
            let error = error.clone();
            let endpoint = endpoint.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_response = Request::get(&endpoint).send().await;
                match fetched_response {
                    Ok(response) => {
                        let json: Result<HelloResponse, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                hello_response.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*hello_response).as_ref() {
        Some(response) => html! {
            <div>
                <p>{ response.name.clone() }</p>
            </div>
        },
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        {"error"} {e}
                        <button onclick={retry}>{"retry"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        {ACTIX_PORT}
                        <button onclick={retry}>{"Call GET "}{endpoint}</button>
                    </>
                }
            }
        },
    }
}

fn main() {
    yew::start_app::<App>();
}
