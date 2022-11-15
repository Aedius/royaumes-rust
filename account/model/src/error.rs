use anyhow::Error;
use derive_more::Display;
use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Responder, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Display)]
pub enum AccountError {
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    AlreadyExist(String),
    #[response(status = 500)]
    WrongQuantity(String),
    #[response(status = 500)]
    Other(String),
}

impl From<anyhow::Error> for AccountError {
    fn from(_: Error) -> Self {
        Self::Other("Oupsi".to_string())
    }
}
