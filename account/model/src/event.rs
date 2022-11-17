use event_model::Event;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountEvent {
    Logged(LoggedIn),
    Created(Created),
    AccountAdded(Quantity),
    AccountRemoved(Quantity),
    Joined(ServerAccount),
    Leaved(ServerAccount),
}

impl Event for AccountEvent {
    fn name_prefix() -> &'static str {
        "account"
    }

    fn event_name(&self) -> &str {
        match self {
            AccountEvent::Logged(_) => "Logged",
            AccountEvent::Created(_) => "Created",
            AccountEvent::AccountAdded(_) => "QuantityAdded",
            AccountEvent::AccountRemoved(_) => "QuantityRemoved",
            AccountEvent::Joined(_) => "JoinedServer",
            AccountEvent::Leaved(_) => "LeavedServer",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Created {
    pub uuid: Uuid,
    pub pseudo: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Quantity {
    pub nb: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoggedIn {
    pub time: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerAccount {
    pub server_id: String,
    pub account_id: String,
}
