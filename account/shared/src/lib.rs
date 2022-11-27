use serde::{Deserialize, Serialize};
use state::Command;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountDto {
    pub pseudo: String,
    pub reputation: usize,
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
    AddReputation(usize),
    RemoveReputation(usize),
}

impl Command for AccountCommand {
    fn command_name(&self) -> &str {
        match self {
            AccountCommand::CreateAccount(_) => "Create",
            AccountCommand::AddReputation(_) => "AddReputation",
            AccountCommand::RemoveReputation(_) => "RemoveReputation",
            AccountCommand::Login(_) => "Login",
        }
    }
}
