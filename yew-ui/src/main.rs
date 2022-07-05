use reqwasm::http::Request;
use types::HelloResponse;
use yew::prelude::*;

// This is read at compile time, please restart ./start_dev.sh if you change this value.
const ACTIX_PORT: &str = std::env!("ACTIX_PORT");

#[function_component(App)]
fn app_component() -> Html {
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
