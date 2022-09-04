mod login;
mod register;

use crate::login::LoginForm;
use crate::register::RegisterForm;
use bounce::BounceRoot;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use weblog::console_info;
use yew::prelude::*;

pub struct Game {
    token: Option<String>,
    menu: Menu,
}

pub enum Msg {
    TokenChange(Option<String>),
    Logout,
    Menu(Menu),
}

pub enum Menu {
    None,
    Login,
    Register,
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = match LocalStorage::get::<String>("token") {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        if token.is_some() {
            let token = token.clone().unwrap();
            spawn_local(async move {
                let message = Request::get("http://127.0.0.1:8000/api/account")
                    .header("Authorization", format!("Bearer {}", token).as_str())
                    .send()
                    .await
                    .unwrap();

                if message.status() == 200 {
                    console_info!(message.text().await.unwrap());
                } else {
                    LocalStorage::clear();
                    window().unwrap().location().reload().unwrap();
                }
            });
        }

        Self {
            token,
            menu: Menu::None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Logout => {
                LocalStorage::clear();
                self.token = None;
                true
            }
            Msg::Menu(menu) => {
                self.menu = menu;
                true
            }
            Msg::TokenChange(token) => {
                self.token = token;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let logout_click = ctx.link().callback(|_| Msg::Logout);
        let no_menu_click = ctx.link().callback(|_| Msg::Menu(Menu::None));
        let login_click = ctx.link().callback(|_| Msg::Menu(Menu::Login));
        let register_click = ctx.link().callback(|_| Msg::Menu(Menu::Register));

        let on_token_change: Callback<Option<String>> = ctx.link().callback(Msg::TokenChange);

        let menu = match self.menu {
            Menu::None => {
                html! {
                    <>
                        <button onclick={login_click}>{ "login" }</button>
                        <button onclick={register_click}>{ "register" }</button>
                    </>
                }
            }
            Menu::Login => {
                html! {
                    <>
                        <button onclick={no_menu_click}>{ "close" }</button>
                        <button onclick={register_click}>{ "register" }</button>
                        <LoginForm {on_token_change}/>
                    </>
                }
            }
            Menu::Register => {
                html! {
                    <>
                        <button onclick={no_menu_click}>{ "close" }</button>
                        <button onclick={login_click}>{ "login" }</button>
                        <RegisterForm {on_token_change}/>
                    </>
                }
            }
        };

        html! {
            <BounceRoot>
                if self.token.is_some(){
                    <div>
                        {"Hello bobi !"}
                        <button onclick={logout_click}>{ "Logout !" }</button>
                        <hero-start token={self.token.clone()}></hero-start>
                    </div>
                }else{
                    <div>
                        <h2>{"inscription"}</h2>
                        {menu}
                    </div>
                }
            </BounceRoot>
        }
    }
}
