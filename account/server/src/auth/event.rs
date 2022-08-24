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
    pub pseudo: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Quantity {
    pub nb: usize,
}
