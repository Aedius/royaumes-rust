use eventstore::EventData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AccountEvent {
    Created(AccountCreated)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AccountCreated {
    pub name: String,
}

impl AccountEvent {
    pub fn to_event_data(&self) -> EventData {
        match self {
            AccountEvent::Created(_) => {
                EventData::json("AccountCreated", &self).unwrap()
            }
        }
    }
}