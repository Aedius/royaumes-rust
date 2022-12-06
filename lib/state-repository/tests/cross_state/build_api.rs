
use state::{ CommandName, Event, EventName};
use state_repository::cross_state::{CrossData, HasTarget};
use state_repository::model_key::ModelKey;
use serde::{Deserialize, Serialize};


pub const PAYMENT_ASKED: &'static str = "payment_asked";
pub const PAYMENT_DONE: &'static str = "payment_done";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PaymentQuestion {
    pub amount: u32,
    pub bank: ModelKey,
}

impl Event for PaymentQuestion {
    fn event_name(&self) -> EventName {
        PAYMENT_ASKED
    }
}

impl HasTarget for PaymentQuestion{
    fn get_target(&self) -> ModelKey {
        self.bank.clone()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PaymentResponse {
    pub amount: u32,
    pub response: ModelKey,
}

impl Event for PaymentResponse {
    fn event_name(&self) -> EventName {
        PAYMENT_DONE
    }
}

impl HasTarget for PaymentResponse{
    fn get_target(&self) -> ModelKey {
        self.response.clone()
    }
}

pub struct PublicBuild {}

impl CrossData for PublicBuild {
    type Question = PaymentQuestion;
    type Answer = PaymentResponse;

    fn question_names() -> Vec<EventName> {
        vec![PAYMENT_ASKED]
    }

    fn answer_names() -> Vec<CommandName> {
        vec![PAYMENT_DONE]
    }
}
