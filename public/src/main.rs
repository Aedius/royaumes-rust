use account_client::Game;
use bounce::helmet::HelmetBridge;
use bounce::BounceRoot;
use stylist::{css, yew::Global};
use yew::prelude::*;

use web_comp::WebComp;

mod web_comp;

struct Body;

impl Component for Body {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <BounceRoot>
            <HelmetBridge default_title="Royaumes-rs"/>
            <WebComp />
            <Global css={css!(
            r#"
                html, body {

                }
            "#
            )} />
            <Game />

        </BounceRoot>
        }
    }
}

fn main() {
    yew::start_app::<Body>();
}
