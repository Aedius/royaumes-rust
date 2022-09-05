mod login;
mod register;

use crate::login::LoginForm;
use crate::register::RegisterForm;
use account_api::AccountDto;
use bounce::BounceRoot;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

pub struct Game {
    token: Option<String>,
    menu: Menu,
    pseudo: Option<String>,
}

pub enum Msg {
    TokenChange(Option<String>),
    Logout,
    Menu(Menu),
    Pseudo(String),
}

pub enum Menu {
    None,
    Login,
    Register,
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let token = match LocalStorage::get::<String>("token") {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        if token.is_some() {
            let token = token.clone().unwrap();

            Self::get_account(ctx, token);
        }

        Self {
            token,
            menu: Menu::None,
            pseudo: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Logout => {
                LocalStorage::clear();
                self.token = None;
                self.pseudo = None;
                true
            }
            Msg::Menu(menu) => {
                self.menu = menu;
                true
            }
            Msg::TokenChange(token) => {
                self.token = token.clone();
                if let Some(t) = token {
                    Self::get_account(ctx, t);
                } else {
                    self.pseudo = None;
                }
                true
            }
            Msg::Pseudo(ps) => {
                self.pseudo = Some(ps);
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
                        if let Some(p) = &self.pseudo{
                            {"Hello "}{p}{" !!"}
                        }else{
                            {"🕰 loading 🕰"}
                        }
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

impl Game {
    fn get_account(ctx: &Context<Game>, token: String) {
        let on_response = ctx.link().callback(Msg::Pseudo);
        spawn_local(async move {
            let message = Request::get("http://127.0.0.1:8000/api/account")
                .header("Authorization", format!("Bearer {}", token).as_str())
                .send()
                .await
                .unwrap();

            if message.status() == 200 {
                let dto: AccountDto = serde_json::from_str(&message.text().await.unwrap()).unwrap();

                on_response.emit(dto.pseudo);
            } else {
                LocalStorage::clear();
                window().unwrap().location().reload().unwrap();
            }
        });
    }
}
