use bounce::helmet::Helmet;
use yew::prelude::*;

pub struct WebComp;

impl Component for WebComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Helmet>
                { Self::script("http://127.0.0.1:8001/hero.js") }
            </Helmet>
        }
    }
}

impl WebComp {
    fn script(name: &str) -> Html {
        html! {
            <script type="module">{format!("
                import init, {{ run }} from '{}';
                async function main() {{
                    await init();
                    run();
                }}
                main();
                ",name)
            }</script>
        }
    }
}
