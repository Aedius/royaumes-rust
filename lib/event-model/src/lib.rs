use anyhow::Result;

use serde::de::DeserializeOwned;
use serde::{Serialize};
use std::fmt::Debug;


pub trait Command: Serialize + DeserializeOwned {
    fn name_prefix() -> &'static str;
    fn command_name(&self) -> &str;
}

pub trait Event: Serialize + DeserializeOwned {
    fn name_prefix() -> &'static str;
    fn event_name(&self) -> &str;
}

pub trait State: Default + Serialize + DeserializeOwned + Debug {
    type Event: Event;
    type Command: Command;

    fn play_event(&mut self, event: &Self::Event);

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>>;

    fn get_position(&self) -> u64;

    fn set_position(&mut self, pos: u64);

    fn state_cache_interval() -> Option<u64>;
}
