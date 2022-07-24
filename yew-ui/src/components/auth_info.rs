use yew::prelude::*;
use yew_oauth2::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContextProps {
    pub auth: OAuth2Context,
}

#[function_component(ViewAuthContext)]
pub fn view_context(props: &ContextProps) -> Html {
    html!(
        <dl>
            <dt> { "Context" } </dt>
            <dd>
                <code><pre>
                    { format!("{:#?}", props.auth) }
                </pre></code>
            </dd>
        </dl>
    )
}

#[function_component(ViewAuthInfo)]
pub fn view_info() -> Html {
    let auth = use_context::<OAuth2Context>();

    html!(
        if let Some(auth) = auth {
            <ViewAuthContext {auth} />
        } else {
            { "context not found." }
        }
    )
}
