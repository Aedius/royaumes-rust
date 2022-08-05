use eventstore::EventData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountEvent {
    Created(Created),
    Added(Quantity),
    Removed(Quantity),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Created {
    pub uuid: Uuid,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Quantity {
    pub nb: usize,
}

impl AccountEvent {
    pub fn to_event_data(&self) -> EventData {
        match self {
            AccountEvent::Created(_) => EventData::json("AccountCreated", &self).unwrap(),
            AccountEvent::Added(_) => EventData::json("QuantityAdded", &self).unwrap(),
            AccountEvent::Removed(_) => EventData::json("QuantityRemoved", &self).unwrap(),
        }
    }
}
