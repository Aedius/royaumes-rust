use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDto {
    pub uuid: String,
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
