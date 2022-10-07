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
