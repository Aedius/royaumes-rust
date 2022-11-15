use crate::concurrent::ConcurrentEvent::TimeTaken;
use event_model::{Command, Event, State};
use serde::{Deserialize, Serialize};
use std::{thread, time};

#[derive(Deserialize, Serialize)]
pub enum ConcurrentCommand {
    TakeTime(u8, String),
}

impl Command for ConcurrentCommand {
    fn name_prefix() -> &'static str {
        "conc.command"
    }

    fn command_name(&self) -> &str {
        match &self {
            ConcurrentCommand::TakeTime(_, _) => "take_time",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum ConcurrentEvent {
    TimeTaken(String),
}

impl Event for ConcurrentEvent {
    fn name_prefix() -> &'static str {
        "conc.event"
    }

    fn event_name(&self) -> &str {
        match &self {
            ConcurrentEvent::TimeTaken(_) => "time_taken",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ConcurrentState {
    pub names: Vec<String>,
    pub position: u64,
}

impl State for ConcurrentState {
    type Event = ConcurrentEvent;
    type Command = ConcurrentCommand;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            ConcurrentEvent::TimeTaken(name) => {
                self.names.push(name.clone());
            }
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        match command {
            ConcurrentCommand::TakeTime(time, name) => {
                let wait = time::Duration::from_millis((100 * time) as u64);

                thread::sleep(wait);

                Ok(vec![TimeTaken(name.clone())])
            }
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
