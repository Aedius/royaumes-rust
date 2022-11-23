use anyhow::Result;
use landtish_shared::LandtishCommand;
use rocket::serde::{Deserialize, Serialize};
use state::State;

use crate::LandtishEvent;

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandtishState {
    nb_player: u64,
    position: u64,
}

impl LandtishState {
    pub fn nb_player(&self) -> u64 {
        self.nb_player
    }
}

impl State for LandtishState {
    type Event = LandtishEvent;
    type Command = LandtishCommand;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            LandtishEvent::Joined => {}
            LandtishEvent::Leaved => {}
        }
    }

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            LandtishCommand::Join(_) => Ok(vec![LandtishEvent::Joined]),
            LandtishCommand::Leave(_) => Ok(vec![LandtishEvent::Leaved]),
        }
    }

    fn get_position(&self) -> u64 {
        self.position
    }

    fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    fn state_cache_interval() -> Option<u64> {
        Some(1)
    }
}
