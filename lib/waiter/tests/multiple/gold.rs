use crate::multiple::Cost;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GoldCommand {
    Pay(Cost),
}

impl Command for GoldCommand {
    fn name_prefix() -> &'static str {
        "worker"
    }

    fn command_name(&self) -> &str {
        use GoldCommand::*;
        match &self {
            Pay(_) => "Pay",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GoldEvent {
    Paid(Cost),
}

impl Event for GoldEvent {
    fn name_prefix() -> &'static str {
        "worker"
    }

    fn event_name(&self) -> &str {
        use GoldEvent::*;

        match &self {
            Paid(_) => "paid",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GoldState {
    nb: u32,
    pub position: u64,
}

impl Default for GoldState {
    fn default() -> Self {
        GoldState {
            nb: 1000,
            position: 0,
        }
    }
}

impl State for GoldState {
    type Event = GoldEvent;
    type Command = GoldCommand;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            GoldEvent::Paid(c) => self.nb -= c.gold,
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        match command {
            GoldCommand::Pay(n) => Ok(vec![GoldEvent::Paid(*n)]),
        }
    }

    fn get_position(&self) -> u64 {
        self.position
    }

    fn set_position(&mut self, pos: u64) {
        self.position = pos;
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}
