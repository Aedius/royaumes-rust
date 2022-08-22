use bounce::BounceRoot;

use reqwasm::http::Request;
use stylist::{css, yew::Global};

use crate::header::Header;
use weblog::console_info;
use yew::prelude::*;

mod header;

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
            <Header />

            <div class={"second-app"}></div>
            <script type={"module"}>{"import init from 'http://127.0.0.1:8000/account-client.js';init('http://127.0.0.1:8000/account-client_bg.wasm');"}</script>

        </BounceRoot>
    }
}

fn main() {
    let document = gloo_utils::document();
    let element = document.query_selector(".first-app").unwrap().unwrap();
    yew::start_app_in_element::<Body>(element);
}
