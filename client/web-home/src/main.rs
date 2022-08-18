use crate::register::RegisterForm;
use bounce::BounceRoot;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use stylist::{css, yew::Global};
use weblog::console_info;
use yew::prelude::*;

mod register;

struct Header {
    token: Option<String>,
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = match LocalStorage::get::<String>("token") {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        console_info!(format!(
            "Hello {}",
            token.clone().unwrap_or_else(|| "world".to_string())
        ));

        Self { token }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                if self.token.is_some(){
                    <div>
                        {"Hello !"}
                    </div>
                }else{
                    <div>
                        <h2>{"inscription"}</h2>
                        <RegisterForm />
                    </div>
                }
            </>
        }
    }
}

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
        </BounceRoot>
    }
}

fn main() {
    yew::start_app::<Body>();
}
