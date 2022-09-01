mod web_comp;

use bounce::helmet::HelmetBridge;
use bounce::BounceRoot;
use stylist::{css, yew::Global};
use web_comp::WebComp;
use web_sys::window;
use yew::prelude::*;

use wasm_bindgen::closure::Closure;
use weblog::console_info;

use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsCast;

struct Body {
    _callback: Closure<dyn Fn()>,
    token: String,
}

enum Msg {
    Reload,
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = ctx.link().callback(|_| Msg::Reload);

        let closure =
            Closure::<dyn Fn()>::new(move || match LocalStorage::get::<String>("reload") {
                Ok(val) => {
                    if val == "1" {
                        LocalStorage::set("reload", "0").unwrap();
                        cb.emit(());
                    }
                }
                Err(err) => {
                    console_info!(format!("{:?}", err));
                }
            });

        let window = window().unwrap();
        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                100,
            )
            .unwrap();

        Self {
            _callback: closure,
            token: LocalStorage::get::<String>("token").unwrap_or_else(|_| "".to_string()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reload => {
                let new_token =
                    LocalStorage::get::<String>("token").unwrap_or_else(|_| "".to_string());

                if self.token != new_token {
                    self.token = new_token;
                    console_info!("has change !!");
                    return true;
                }
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
            <account-login token={self.token.clone()}></account-login>
            if !self.token.is_empty() {
                <hero-start></hero-start>
            }

        </BounceRoot>
        }
    }
}

fn main() {
    yew::start_app::<Body>();
}
