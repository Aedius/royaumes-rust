#![allow(dead_code)]
#![allow(unused_variables)]

use weblog::console_info;
use yew::{html, Component, Context, Html, Properties};

pub struct Start {
    token: Option<String>,
}

pub enum Msg {
    TokenChange(Option<String>),
}

#[derive(PartialEq, Eq, Properties)]
pub struct Props {
    pub token: Option<String>,
}

impl Component for Start {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        match &ctx.props().token {
            None => Self { token: None },
            Some(t) => Self {
                token: Some(t.clone()),
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TokenChange(token) => {
                self.token = token;
                console_info!("hero TokenChange");
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>
            if self.token.is_some(){
                {self.token.clone().unwrap()}
            }else{
                {"no token yet"}
            }
            </p>
        }
    }
}
