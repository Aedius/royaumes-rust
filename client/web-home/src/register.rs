use crate::{function_component, Request};
use api_account::{AccountCommand, CreateAccount};
use bounce::{use_atom, Atom};
use gloo_storage::{LocalStorage, Storage};
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
    token: Option<String>,
}

impl Default for Register {
    fn default() -> Self {
        let token = match LocalStorage::get::<String>("token") {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        Self {
            pseudo: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            password_ok: false,
            token,
        }
    }
}

#[function_component(RegisterForm)]
pub fn register_setter() -> Html {
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
                token: to_register.token.clone(),
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
                token: to_register.token.clone(),
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
                token: to_register.token.clone(),
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
                token: to_register.token.clone(),
            });
        })
    };

    let on_register = {
        let to_register = to_register.clone();
        Callback::from(move |_: MouseEvent| {
            let create_account = AccountCommand::CreateAccount(CreateAccount {
                pseudo: to_register.pseudo.clone(),
                email: to_register.email.clone(),
                password: to_register.password.clone(),
            });

            let to_register = to_register.clone();
            spawn_local(async move {
                let resp = Request::post("http://localhost:8000/auth/")
                    .body(serde_json::to_string(&create_account).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();

                if resp.ok() {
                    let token = resp.text().await.unwrap();
                    LocalStorage::set("token", token.clone()).unwrap();

                    to_register.set(Register {
                        pseudo: to_register.pseudo.clone(),
                        email: to_register.email.clone(),
                        password: to_register.password.clone(),
                        password_ok: to_register.password_ok,
                        token: Some(token),
                    });
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
            if to_register.token.is_some(){
                <p>{"You have to logout to create an account."}</p>
            }else{
                <label>{"pseudo"}<br/>
                    <input type="text" oninput={on_pseudo_input} value={to_register.pseudo.to_string()} />
                </label><br/>
                <label>{"email"}<br/>
                    <input type="text" oninput={on_email_input} value={to_register.email.to_string()} />
                </label><br/>
                <label>{"password"}<br/>
                    <input type="password" oninput={on_password_input} value={to_register.password.to_string()} />
                </label><br/>
                <label>{"password check"}<br/>
                    <input type="password" oninput={on_password_check_input} />
                </label>
                if to_register.password_ok {
                    {"✅"}
                }else{
                    {"❌"}
                }
                <br/>
                if can_register{
                    <button onclick={on_register}>{"go"}</button>
                }
            }
        </div>
    }
}
