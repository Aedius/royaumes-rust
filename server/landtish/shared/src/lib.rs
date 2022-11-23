use serde::{Deserialize, Serialize};
use state::Command;

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
    fn name_prefix() -> &'static str {
        "landtish"
    }

    fn command_name(&self) -> &str {
        match self {
            LandtishCommand::Join(_) => "join",
            LandtishCommand::Leave(_) => "leave",
        }
    }
}
