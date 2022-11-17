use serde::{Deserialize, Serialize};
use state::Command;

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
    pub time: u64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerAccount {
    pub server_id: String,
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AccountCommand {
    CreateAccount(CreateAccount),
    Login(Login),
    AddQuantity(usize),
    RemoveQuantity(usize),
    Join(ServerAccount),
    Leave(ServerAccount),
}

impl Command for AccountCommand {
    fn name_prefix() -> &'static str {
        "account"
    }

    fn command_name(&self) -> &str {
        match self {
            AccountCommand::CreateAccount(_) => "Create",
            AccountCommand::AddQuantity(_) => "AddQuantity",
            AccountCommand::RemoveQuantity(_) => "RemoveQuantity",
            AccountCommand::Login(_) => "Login",
            AccountCommand::Join(_) => "JoinServer",
            AccountCommand::Leave(_) => "LeaveServer",
        }
    }
}
