use serde::{Deserialize, Serialize};
use state::{Command, CommandName};

pub mod flow_create;
pub mod flow_payment;
pub mod flow_production_change;

pub const STREAM_NAME :&str= "bank";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Info {
    pub pseudo: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BankCommand {
    Join(Info),
    Leave(Info),
}

impl Command for BankCommand {

    fn command_name(&self) -> CommandName {
        match self {
            BankCommand::Join(_) => "join",
            BankCommand::Leave(_) => "leave",
        }
    }
}
