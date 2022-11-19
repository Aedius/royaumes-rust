use bounce::helmet::Helmet;
use global_config::Components::Public;
use global_config::Config;
use yew::prelude::*;

pub struct WebComp {
    scripts: Vec<String>,
}

impl Component for WebComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config::load();

        Self {
            scripts: config.get_scripts(&Public),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let scrips: Vec<Html> = self.scripts.iter().map(|s| Self::script(s)).collect();
        html! {
            <Helmet>
                { scrips }
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
