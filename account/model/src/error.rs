use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Responder, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
