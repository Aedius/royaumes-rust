use account_shared::{AccountCommand, CreateAccount};
use bounce::{use_atom, Atom};
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::prelude::*;
use yew::Callback;

#[derive(Eq, PartialEq, Atom)]
struct Register {
    pseudo: String,
    email: String,
    password: String,
    password_ok: bool,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            pseudo: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            password_ok: false,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_token_change: Callback<Option<String>>,
}

#[function_component(RegisterForm)]
pub fn register_setter(props: &Props) -> Html {
    let to_register = use_atom::<Register>();

    let on_pseudo_input = {
        let to_register = to_register.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_register.set(Register {
                pseudo: input.value(),
                email: to_register.email.clone(),
                password: to_register.password.clone(),
                password_ok: false,
            });
        })
    };
    let on_email_input = {
        let to_register = to_register.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_register.set(Register {
                pseudo: to_register.pseudo.clone(),
                email: input.value(),
                password: to_register.password.clone(),
                password_ok: false,
            });
        })
    };
    let on_password_input = {
        let to_register = to_register.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_register.set(Register {
                pseudo: to_register.pseudo.clone(),
                email: to_register.email.clone(),
                password: input.value(),
                password_ok: false,
            });
        })
    };
    let on_password_check_input = {
        let to_register = to_register.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            let ok =
                to_register.password.len() > 1 && to_register.password.clone() == input.value();

            to_register.set(Register {
                pseudo: to_register.pseudo.clone(),
                email: to_register.email.clone(),
                password: to_register.password.clone(),
                password_ok: ok,
            });
        })
    };

    let on_register = {
        let to_register = to_register.clone();
        let on_token_change = props.on_token_change.clone();

        Callback::from(move |_: MouseEvent| {
            let create_account = AccountCommand::CreateAccount(CreateAccount {
                pseudo: to_register.pseudo.clone(),
                email: to_register.email.clone(),
                password: to_register.password.clone(),
            });

            let on_token_change = on_token_change.clone();
            spawn_local(async move {
                let resp = Request::post("http://127.0.0.1:8000/api/")
                    .body(serde_json::to_string(&create_account).unwrap())
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

    let can_register = to_register.pseudo.len() > 1
        && to_register.email.len() > 1
        && to_register.password.len() > 1
        && to_register.password_ok;

    html! {
        <div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Pseudo :"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    <input type="name" oninput={on_pseudo_input} value={to_register.pseudo.to_string()} />
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Email :"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    <input type="email" oninput={on_email_input} value={to_register.email.to_string()} />
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Password :"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    <input type="password" oninput={on_password_input} value={to_register.password.to_string()} />
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                    <p class="m-0">{"Password check :"}</p>
                </div>
                <div class="col-xs-9 level-item">
                    if to_register.password_ok {
                        <input class="text-success input-success" type="password" oninput={on_password_check_input} />
                    }else{
                        <input class="text-danger input-error" type="password" oninput={on_password_check_input} />
                    }
                </div>
            </div>
            <div class="row level">
                <div class="col-xs-3 level-item">
                </div>
                <div class="col-xs-9 level-item">
                    if can_register{
                        <button class="outline btn-primary" onclick={on_register}>{"register"}</button>
                    }
                </div>
            </div>
        </div>
    }
}
