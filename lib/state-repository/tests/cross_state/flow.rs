use serde::{Deserialize, Serialize};
use state::{Event, EventName, StateName};
use state_repository::cross_state::CrossData;
use state_repository::model_key::ModelKey;
use crate::cross_state::build::BUILD_STATE_NAME;
use crate::cross_state::gold::GOLD_STATE_NAME;

const PAYMENT_NEEDED: &'static str = "payment_needed";
const PAYMENT_DONE: &'static str = "payment_done";

pub struct CrossPayment {}

impl CrossData for CrossPayment {
    type Question = PaymentNeeded;
    type Answer = PaymentDone;

    fn questioner() -> StateName {
        BUILD_STATE_NAME
    }

    fn answerer() -> StateName {
        GOLD_STATE_NAME
    }

    fn question_names() -> Vec<EventName> {
        vec!(PAYMENT_NEEDED)
    }

    fn answer_names() -> Vec<EventName> {
        vec!(PAYMENT_DONE)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaymentNeeded {
    amount: u32,
}

impl Event for NeedPayment {
    fn event_name(&self) -> EventName {
        PAYMENT_NEEDED
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PaymentResult {
    Done(u32),
    NotDone,
}

impl Event for PaymentDone {
    fn event_name(&self) -> EventName {
        PAYMENT_DONE
    }
}

