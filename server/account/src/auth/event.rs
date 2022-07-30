use eventstore::EventData;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AccountEvent {
    Created(Created),
    Added(Quantity)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Created {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Quantity {
    pub nb: usize,

}

impl AccountEvent {
    pub fn to_event_data(&self) -> EventData {
        match self {
            AccountEvent::Created(_) => {
                EventData::json("AccountCreated", &self).unwrap()
            }
            AccountEvent::Added(_) => {
                EventData::json("QuantityAdded", &self).unwrap()
            }
        }
    }
}