use anyhow::Error;
use derive_more::Display;
use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Responder, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Display)]
pub enum LandtishError {
    #[response(status = 500)]
    Other(String),
}

impl From<anyhow::Error> for LandtishError {
    fn from(_: Error) -> Self {
        Self::Other("Oupsi".to_string())
    }
}
