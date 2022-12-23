use serde::{Deserialize, Serialize};
use state::{CommandName, Event, EventName};
use state_repository::cross_state::{CrossData, HasTarget};
use state_repository::model_key::ModelKey;

pub const PRODUCTION_CHANGED: &'static str = "production_changed";
pub const PRODUCTION_CHANGED_REGISTERED: &'static str = "production_changed_registered";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProductionChangedQuestion {
    pub amount: u32,
    pub bank: ModelKey,
}

impl Event for ProductionChangedQuestion {
    fn event_name(&self) -> EventName {
        PRODUCTION_CHANGED
    }
}

impl HasTarget for ProductionChangedQuestion {
    fn get_target(&self) -> ModelKey {
        self.bank.clone()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProductionChangedResponse {
    pub amount: u32,
    pub response: ModelKey,
}

impl Event for ProductionChangedResponse {
    fn event_name(&self) -> EventName {
        PRODUCTION_CHANGED_REGISTERED
    }
}

impl HasTarget for ProductionChangedResponse {
    fn get_target(&self) -> ModelKey {
        self.response.clone()
    }
}

pub struct PublicBuild {}

impl CrossData for PublicBuild {
    type Question = ProductionChangedQuestion;
    type Answer = ProductionChangedResponse;

    fn question_names() -> Vec<EventName> {
        vec![PRODUCTION_CHANGED]
    }

    fn answer_names() -> Vec<CommandName> {
        vec![PRODUCTION_CHANGED_REGISTERED]
    }
}
