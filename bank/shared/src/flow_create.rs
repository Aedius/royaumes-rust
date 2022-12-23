use serde::{Deserialize, Serialize};
use uuid::Uuid;
use state::{CommandName, Event, EventName};
use state_repository::cross_state::{CrossData, HasTarget};
use state_repository::model_key::ModelKey;
use crate::STREAM_NAME;


pub const CREATE_BANK_REQUIRED: &'static str = "create_bank_required";
pub const BANK_CREATED: &'static str = "bank_created";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BankCreateQuestion {
    pub gold: u32,
}

impl Event for BankCreateQuestion {
    fn event_name(&self) -> EventName {
        CREATE_BANK_REQUIRED
    }
}

impl HasTarget for PaymentQuestion {
    fn get_target(&self) -> ModelKey {
        ModelKey::new(STREAM_NAME.to_string(), Uuid::new_v4().to_string())
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BankCreatedResponse {
    pub response: ModelKey,
}

impl Event for BankCreatedResponse {
    fn event_name(&self) -> EventName {
        BANK_CREATED
    }
}

impl HasTarget for PaymentResponse {
    fn get_target(&self) -> ModelKey {
        self.response.clone()
    }
}

pub struct BankCreate {}

impl CrossData for BankCreate {
    type Question = BankCreateQuestion;
    type Answer = BankCreatedResponse;

    fn question_names() -> Vec<EventName> {
        vec![CREATE_BANK_REQUIRED]
    }

    fn answer_names() -> Vec<EventName> {
        vec![BANK_CREATED]
    }
}
