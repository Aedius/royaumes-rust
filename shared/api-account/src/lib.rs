use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDto {
    pub uuid: Uuid,
    pub nb: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateAccount {
    pub pseudo: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountCommand {
    CreateAccount(CreateAccount),
    AddQuantity(usize),
    RemoveQuantity(usize),
}
