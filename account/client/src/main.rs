use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main() {
    let document = gloo_utils::document();
    let element = document.query_selector(".second-app").unwrap().unwrap();
    yew::start_app_in_element::<App>(element);
}
