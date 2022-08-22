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
            <ce-yew></ce-yew>
        </BounceRoot>
    }
}

fn main() {
    yew::start_app::<Body>();
}
