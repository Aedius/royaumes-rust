use anyhow::Result;
use bank_shared::bankCommand;
use rocket::serde::{Deserialize, Serialize};
use state::{State, StateName};

use crate::bankEvent;

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct bankState {
    nb_player: u64,
    position: u64,
}

impl bankState {
    pub fn nb_player(&self) -> u64 {
        self.nb_player
    }
}

impl State for bankState {
    type Event = bankEvent;
    type Command = bankCommand;

    fn name_prefix() -> StateName {
        "bank"
    }

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            bankEvent::Joined => {}
            bankEvent::Leaved => {}
        }
    }

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            bankCommand::Join(_) => Ok(vec![bankEvent::Joined]),
            bankCommand::Leave(_) => Ok(vec![bankEvent::Leaved]),
        }
    }
    fn state_cache_interval() -> Option<u64> {
        Some(1)
    }
}
