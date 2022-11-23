use serde::{Deserialize, Serialize};
use state::Event;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LandtishEvent {
    Joined,
    Leaved,
}

impl Event for LandtishEvent {
    fn name_prefix() -> &'static str {
        "account"
    }

    fn event_name(&self) -> &str {
        match self {
            LandtishEvent::Joined => "joined",
            LandtishEvent::Leaved => "leaved",
        }
    }
}
