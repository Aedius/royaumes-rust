use crate::{function_component, Request};
use account_api::{AccountCommand, Login as LoginCmd};
use bounce::{use_atom, Atom};
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::prelude::*;
use yew::Callback;

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

#[function_component(LoginForm)]
pub fn login_setter() -> Html {
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
        Callback::from(move |_: MouseEvent| {
            let login_account = AccountCommand::Login(LoginCmd {
                email: to_login.email.clone(),
                password: to_login.password.clone(),
            });

            spawn_local(async move {
                let resp = Request::post("http://127.0.0.1:8000/auth/")
                    .body(serde_json::to_string(&login_account).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();

                if resp.ok() {
                    let token = resp.text().await.unwrap();
                    LocalStorage::set("token", token).unwrap();

                    let window = window().unwrap();
                    window.location().reload().unwrap();
                }
            });
        })
    };

    let can_login = to_login.email.len() > 1 && to_login.password.len() > 1;

    html! {
        <div>

            <label>{"email"}<br/>
                <input type="text" oninput={on_email_input} value={to_login.email.to_string()} />
            </label><br/>
            <label>{"password"}<br/>
                <input type="password" oninput={on_password_input} value={to_login.password.to_string()} />
            </label><br/>

            if can_login{
                <button onclick={on_login}>{"go"}</button>
            }
        </div>
    }
}
