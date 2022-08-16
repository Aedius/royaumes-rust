use bounce::{use_atom, use_atom_value, Atom, BounceRoot};
use stylist::{css, yew::Global};
use web_sys::HtmlInputElement;
use weblog::*;
use yew::prelude::*;

enum Msg {
    Pseudo(String),
    Email(String),
    Password(String),
    PasswordVerify(String),
    Send,
}

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

#[function_component(RegisterReader)]
fn register_reader() -> Html {
    let to_register = use_atom_value::<Register>();

    html! {
        <div>
        {"Hello, "}{&to_register.pseudo}{", "}{&to_register.email}{" : "}{&to_register.password}
        </div>
    }
}

#[function_component(RegisterSetter)]
fn register_setter() -> Html {
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

    let on_password2_input = {
        let to_register = to_register.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            to_register.set(Register {
                pseudo: to_register.pseudo.clone(),
                email: to_register.email.clone(),
                password: to_register.password.clone(),
                password_ok: to_register.password.clone() == input.value(),
            });
        })
    };

    html! {
        <div>
            <input type="text" oninput={on_pseudo_input} value={to_register.pseudo.to_string()} />
            <input type="text" oninput={on_email_input} value={to_register.email.to_string()} />
            <input type="password" oninput={on_password_input} value={to_register.password.to_string()} />
            <input type="password" oninput={on_password2_input} />
            if to_register.password_ok {
                {"✅"}
            }else{
                {"❌"}
            }
        </div>
    }
}

impl Component for Register {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console_info!("Hello world");

        Self {
            pseudo: "aa".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            password_ok: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Pseudo(p) => {
                if p != self.pseudo {
                    self.pseudo = p;
                    true
                } else {
                    false
                }
            }
            Msg::Email(e) => {
                if e != self.email {
                    self.email = e;
                    true
                } else {
                    false
                }
            }
            Msg::Password(p) => {
                if p != self.password {
                    self.password = p;
                    true
                } else {
                    false
                }
            }
            Msg::PasswordVerify(p) => {
                let verify = p == self.password;
                if verify != self.password_ok {
                    self.password_ok = verify;
                    true
                } else {
                    false
                }
            }
            Msg::Send => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BounceRoot>
                 <Global css={css!(
                    r#"
                        html, body {
                            font-family: sans-serif;
                            padding: 0;
                            margin: 0;
                            min-height: 100vh;
                            flex-direction: column;
                            background-color: #333;
                            color:white;
                        }
                    "#
                )} />
                <div>
                    <h2>{"inscription"}</h2>
                    <RegisterSetter />
                    <RegisterReader />
                </div>
            </BounceRoot>
        }
    }
}

fn main() {
    yew::start_app::<Register>();
}
