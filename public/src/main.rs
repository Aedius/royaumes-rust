use bounce::BounceRoot;

use stylist::{css, yew::Global};
use yew::prelude::*;

#[function_component(Body)]
fn body() -> Html {
    html! {
        <BounceRoot>
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

        </BounceRoot>
    }
}

fn main() {
    yew::start_app::<Body>();
}
