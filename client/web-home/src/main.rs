use stylist::{css, yew::Global};
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
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
                <div>
                    <h2>{"inscription"}</h2>
                    <input placeholder={"pseudo"}/>
                    <input placeholder={"email"}/>
                    <input placeholder={"password"}/>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "let's go !" }</button>
                    <p>{ self.value }</p>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
