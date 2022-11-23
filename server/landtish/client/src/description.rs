use yew::prelude::*;

pub struct Description {}

impl Component for Description {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Description {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {"Some description TBD"}
            </div>
        }
    }
}
