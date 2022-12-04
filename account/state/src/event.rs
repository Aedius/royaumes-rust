use serde::{Deserialize, Serialize};
use state::Event;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountEvent {
    Logged(LoggedIn),
    Created(Created),
    ReputationAdded(usize),
    ReputationRemoved(usize),
}

impl Event for AccountEvent {
    fn event_name(&self) -> &'static str {
        match self {
            AccountEvent::Logged(_) => "Logged",
            AccountEvent::Created(_) => "Created",
            AccountEvent::ReputationAdded(_) => "ReputationAdded",
            AccountEvent::ReputationRemoved(_) => "ReputationRemoved",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Created {
    pub uuid: Uuid,
    pub pseudo: String,
    pub time: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoggedIn {
    pub time: u64,
}
