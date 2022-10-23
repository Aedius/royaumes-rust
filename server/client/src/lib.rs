mod start;

use crate::start::{Msg, Props, Start};
use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;

#[derive(Default)]
struct ComponentWrapper {
    content: Option<AppHandle<Start>>,
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        let props = match this.attributes().get_named_item("token") {
            None => Props { token: None },
            Some(a) => Props {
                token: Some(a.value()),
            },
        };

        self.content = Some(yew::start_app_with_props_in_element::<Start>(
            this.clone().into(),
            props,
        ));
    }

    fn shadow() -> bool {
        false
    }

    fn observed_attributes() -> &'static [&'static str] {
        &["token"]
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected server");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected server");
    }

    fn attribute_changed_callback(
        &mut self,
        _this: &HtmlElement,
        name: String,
        old_value: Option<String>,
        new_value: Option<String>,
    ) {
        if name == "token" && old_value != new_value {
            if let Some(content) = &self.content {
                if new_value == Some("".to_string()) {
                    content.send_message(Msg::TokenChange(None));
                } else {
                    content.send_message(Msg::TokenChange(new_value));
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("server-start");
}
