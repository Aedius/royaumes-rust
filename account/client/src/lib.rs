mod header;

use crate::header::{Header, Msg};
use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;

#[derive(Default)]
struct ComponentWrapper {
    content: Option<AppHandle<Header>>,
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        self.content = Some(yew::start_app_in_element::<Header>(this.clone().into()));
    }

    fn shadow() -> bool {
        false
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["token"]
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "token" {
            if let Some(content) = &self.content {
                if new_value == Some("".to_string()) {
                    content.send_message(Msg::TokenChange(None));
                } else {
                    content.send_message(Msg::TokenChange(new_value));
                }
            }
        }
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected");
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("account-login");
}
