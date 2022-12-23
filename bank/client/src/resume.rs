use yew::prelude::*;

pub struct Resume {}

impl Component for Description {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Description {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {"Here the resume"}
            </div>
        }
    }
}
