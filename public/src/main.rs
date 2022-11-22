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
                    background-color: #515F84;
                    color: #7E88AB;
                }
                a, button, .call-to-action {
                    color: white
                }
            "#
            )} />
            <account-login />

            <p>
                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque nec euismod urna. Integer ac vehicula nisi. Donec placerat risus ut justo placerat aliquet. Pellentesque imperdiet arcu sit amet mauris cursus tempor. In nisl mauris, porta et purus sit amet, lacinia congue ex. Maecenas eget eleifend ligula, ut volutpat diam. Cras libero leo, porta id auctor et, cursus vel libero. In hac habitasse platea dictumst. Vivamus eu commodo ipsum. Maecenas vulputate turpis quis elit feugiat pulvinar. Duis ac gravida velit. Duis eu eros odio. Nullam at quam nisl. Mauris at magna augue."}
            </p><p>
                {"Vestibulum eu eros dictum, auctor nisl in, mollis magna. Etiam eu dui mauris. Etiam ac luctus enim. Quisque facilisis eget leo nec convallis. Phasellus consectetur pharetra libero, ut aliquet arcu sollicitudin quis. Suspendisse et tristique justo. Donec dictum tortor ac felis laoreet condimentum. Sed ultricies iaculis turpis, et lobortis quam volutpat quis. Curabitur nec est a dui fringilla porta a quis eros. Morbi malesuada ultrices ligula vitae tincidunt. Praesent ac sollicitudin mauris. Vestibulum convallis enim non tellus condimentum, nec rutrum nulla mollis. Nulla eleifend vel purus nec tristique. Integer et arcu neque. Suspendisse porta finibus metus id vestibulum."}
            </p><p>
                {"Maecenas condimentum, arcu et porttitor ultrices, arcu ex porttitor nisl, ac varius ex lorem nec ligula. Maecenas ut tellus tincidunt, sollicitudin lacus vitae, pulvinar urna. Mauris in massa vitae lorem porta aliquam quis nec elit. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Phasellus placerat odio quam, vitae congue quam egestas vel. Maecenas at cursus arcu. Integer ligula dui, cursus et metus ut, finibus porta orci. Phasellus pulvinar sed est nec vehicula. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Nullam porta diam non ligula faucibus consequat. Etiam cursus nisl est, vitae tempus enim tristique vel. In in mauris condimentum, suscipit eros eu, vehicula erat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae;"}
            </p><p>
                {"Donec vitae metus nibh. Suspendisse quis imperdiet dolor. Aliquam erat volutpat. Fusce at orci mi. Vestibulum et facilisis erat. Suspendisse congue lacus at ligula efficitur luctus. Etiam feugiat pulvinar felis imperdiet volutpat. Integer porttitor, dolor tempus suscipit lobortis, risus ligula mattis erat, eget iaculis nisl leo in eros. Proin ac auctor leo, quis aliquet orci. Nulla eu lorem dapibus, porttitor risus consequat, accumsan purus. Nam eget mi non sapien aliquam congue quis ut ex. Ut molestie elit leo, sit amet tristique est scelerisque ut. Curabitur auctor ullamcorper pulvinar."}
            </p><p>
                {"Sed laoreet tortor a sem convallis interdum. Suspendisse arcu ex, posuere at velit at, auctor interdum nisl. Vestibulum tempus ac neque tristique hendrerit. Suspendisse potenti. Vivamus dapibus elit ut urna sollicitudin, ut feugiat ex auctor. Nunc eget tortor et lectus porttitor luctus sed id nisl. Proin vitae eleifend nulla. Vivamus in dui vel enim eleifend eleifend. Nunc non eros est. Nam condimentum lacus in lectus tempor, ut volutpat erat pulvinar. Suspendisse potenti. Mauris imperdiet semper diam ac interdum. Pellentesque faucibus libero eu tincidunt mattis."}
            </p>

        </BounceRoot>
        }
    }
}

fn main() {
    yew::start_app::<Body>();
}
