mod web_comp;

use bounce::helmet::HelmetBridge;
use bounce::BounceRoot;
use stylist::{css, yew::Global};
use web_comp::WebComp;
use yew::prelude::*;

#[function_component(Body)]
fn body() -> Html {
    html! {
        <BounceRoot>
            <HelmetBridge default_title="Royaumes-rs"/>
            <WebComp></WebComp>
            <Global css={css!(
            r#"
                html, body {
                    font-family: sans-serif;
                    padding: 0;
                    margin: 0;
                    min-height: 100vh;
                    flex-direction: column;
                    background-color: #333;
                    color:white;
                }
            "#
            )} />
            <account-login></account-login>
            <hero-start></hero-start>

        </BounceRoot>
    }
}

fn main() {
    yew::start_app::<Body>();
}
