use serde::{Deserialize, Serialize};
use state::{Command, CommandName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Info {
    pub pseudo: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LandtishCommand {
    Join(Info),
    Leave(Info),
}

impl Command for LandtishCommand {

    fn command_name(&self) -> CommandName {
        match self {
            LandtishCommand::Join(_) => "join",
            LandtishCommand::Leave(_) => "leave",
        }
    }
}
