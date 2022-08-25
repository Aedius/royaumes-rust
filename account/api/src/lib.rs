use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDto {
    pub pseudo: String,
    pub nb: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateAccount {
    pub pseudo: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountCommand {
    CreateAccount(CreateAccount),
    Login(Login),
    AddQuantity(usize),
    RemoveQuantity(usize),
}
