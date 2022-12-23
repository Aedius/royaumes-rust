mod resume;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

use crate::resume::Resume;
use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;

#[derive(Default)]
struct ComponentWrapper {
    content: Option<AppHandle<Resume>>,
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        self.content = Some(yew::Renderer::<Resume>::with_root(this.clone().into()).render());

    }

    fn shadow() -> bool {
        false
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected account");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected account");
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("bank-resume");
}
