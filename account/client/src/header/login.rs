use account_shared::{AccountCommand, Login as LoginCmd};
use bounce::{use_atom, Atom};
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::prelude::*;
use yew::{function_component, html, Callback, Properties};

#[derive(Eq, PartialEq, Atom)]
struct Login {
    email: String,
    password: String,
}

impl Default for Login {
    fn default() -> Self {
        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_token_change: Callback<Option<String>>,
}

#[function_component(LoginForm)]
pub fn login_setter(props: &Props) -> Html {
    let to_login = use_atom::<Login>();

    let on_email_input = {
        let to_login = to_login.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_login.set(Login {
                email: input.value(),
                password: to_login.password.clone(),
            });
        })
    };
    let on_password_input = {
        let to_login = to_login.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_login.set(Login {
                email: to_login.email.clone(),
                password: input.value(),
            });
        })
    };

    let on_login = {
        let to_login = to_login.clone();
        let on_token_change = props.on_token_change.clone();

        Callback::from(move |_: MouseEvent| {
            let login_account = AccountCommand::Login(LoginCmd {
                email: to_login.email.clone(),
                password: to_login.password.clone(),
            });

            let on_token_change = on_token_change.clone();
            spawn_local(async move {
                let resp = Request::post("http://127.0.0.1:8000/api/")
                    .body(serde_json::to_string(&login_account).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();

                if resp.ok() {
                    let token = resp.text().await.unwrap();
                    LocalStorage::set("token", token.clone()).unwrap();

                    on_token_change.emit(Some(token));
                }
            });
        })
    };

    let can_login = to_login.email.len() > 1 && to_login.password.len() > 1;

    html! {
        <div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Email :"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    <input type="email" oninput={on_email_input} value={to_login.email.to_string()} />
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Password:"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    <input type="password" oninput={on_password_input} value={to_login.password.to_string()} />
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                </div>
                <div class="col-xs-9 level-item">
                    if can_login{
                        <button class="call-to-action" onclick={on_login}>{"log me in"}</button>
                    }
                </div>
            </div>

        </div>
    }
}
