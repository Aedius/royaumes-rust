mod web_comp;

use bounce::helmet::HelmetBridge;
use bounce::BounceRoot;
use stylist::{css, yew::Global};
use web_comp::WebComp;
use yew::prelude::*;
use web_sys::window;
use js_sys::Function;
use weblog::console_info;
use js_function_promisify::Callback;
use wasm_bindgen::JsValue;

struct Body{
    callback: Callback<dyn FnMut()->()>
}

enum Msg {
    Reload
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let future = Callback::new(|| {
            console_info!("call !");
            Ok("".into())
        });

        let window = window().unwrap();
        window.set_onstorage(Some(future.as_function().as_ref()));

        Self{
            callback: future
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reload => {
                console_info!("RELOADDDD");
            }
        }
        false
    }


    fn view(&self, ctx: &Context<Self>) -> Html {

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
}


fn main() {
    yew::start_app::<Body>();
}
