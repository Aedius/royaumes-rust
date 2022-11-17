mod login;
mod register;

use crate::header::login::LoginForm;
use crate::header::register::RegisterForm;
use account_shared::AccountDto;
use bounce::BounceRoot;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

pub struct Header {
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

impl Component for Header {
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
                    <ul>
                        <li><div class="tab-item-content" onclick={login_click}>{ "login" }</div></li>
                        <li><div class="tab-item-content" onclick={register_click}>{ "register" }</div></li>
                    </ul>
                }
            }
            Menu::Login => {
                html! {
                    <ul>
                        <li class="selected"><div class="tab-item-content" onclick={no_menu_click}>{ "login" }</div></li>
                        <li><div class="tab-item-content" onclick={register_click}>{ "register" }</div></li>

                    </ul>
                }
            }
            Menu::Register => {
                html! {
                    <ul>
                        <li><div class="tab-item-content" onclick={login_click}>{ "login" }</div></li>
                        <li class="selected"><div class="tab-item-content" onclick={no_menu_click}>{ "register" }</div></li>
                    </ul>
                }
            }
        };

        let content = match self.menu {
            Menu::None => {
                html! {
                    <>
                    </>
                }
            }
            Menu::Login => {
                html! {
                    <>
                        <LoginForm {on_token_change}/>
                    </>
                }
            }
            Menu::Register => {
                html! {
                    <>
                        <RegisterForm {on_token_change}/>
                    </>
                }
            }
        };

        let data = self.pseudo.as_ref().map(|p| (get_initial(p), p));

        html! {
            <BounceRoot>
                if self.token.is_some(){
                    <div>
                        <div class="tab-container tabs--right">
                            <ul>
                                <li>
                                    if let Some(d) = data.clone(){
                                        <div class="avatar avatar--sm text-gray-000" data-text={d.0}></div>
                                    }else{
                                        <div class="card u-flex u-items-center u-justify-center">
                                            <div class="animated loading hide-text">
                                                <p>{"🕰 loading 🕰"}</p>
                                            </div>
                                        </div>
                                    }
                                </li>
                                <li><div class="tab-item-content" onclick={logout_click}>{ "log out" }</div></li>
                            </ul>
                        </div>

                        if let Some(d) = data{
                            {"Hello "}{d.1}{" !!"}
                        }else{
                            <div class="card u-flex u-items-center u-justify-center">
                                <div class="animated loading hide-text">
                                    <p>{"🕰 loading 🕰"}</p>
                                </div>
                            </div>
                        }

                        <server-start></server-start>
                    </div>
                }else{
                    <div>
                        <div class="tab-container tabs--right">
                             {menu}
                        </div>

                        {content}
                    </div>
                }
            </BounceRoot>
        }
    }
}

fn get_initial(s: &str) -> String {
    let mut res: Vec<String> = Vec::new();

    let chars: Vec<char> = s.chars().collect();
    let windows = chars.windows(2);

    for w in windows {
        println! {"[{}, {}]", w[0], w[1]};
        if res.is_empty() {
            res.push(w[0].to_uppercase().to_string());
        } else if w[0] == ' ' {
            res.push(w[1].to_lowercase().to_string());
        }
    }

    res.join("")
}

impl Header {
    fn get_account(ctx: &Context<Header>, token: String) {
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
