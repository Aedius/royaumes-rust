use serde::{Deserialize, Serialize};
use state::{Event, EventName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LandtishEvent {
    Joined,
    Leaved,
}

impl Event for LandtishEvent {

    fn event_name(&self) -> EventName {
        match self {
            LandtishEvent::Joined => "joined",
            LandtishEvent::Leaved => "leaved",
        }
    }
}
