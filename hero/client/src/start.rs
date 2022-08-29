use yew::prelude::*;

pub struct Start;

pub enum Msg {}

impl Component for Start {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>
             {"Lorem Ipsum"}
            </p>
        }
    }
}
