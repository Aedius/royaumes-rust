use serde::{Deserialize, Serialize};
use state::{Event, EventName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum bankEvent {
    Joined,
    Leaved,
}

impl Event for bankEvent {

    fn event_name(&self) -> EventName {
        match self {
            bankEvent::Joined => "joined",
            bankEvent::Leaved => "leaved",
        }
    }
}
