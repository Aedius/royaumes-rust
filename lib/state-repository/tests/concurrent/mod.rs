use crate::concurrent::ConcurrentEvent::TimeTaken;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, State};
use std::{thread, time};

#[derive(Deserialize, Serialize, Clone, Debug)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    type Notification =();

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            ConcurrentEvent::TimeTaken(name) => {
                self.names.push(name.clone());
            }
        }
    }

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>> {
        match command {
            ConcurrentCommand::TakeTime(time, name) => {
                let wait = time::Duration::from_millis((100 * time) as u64);

                thread::sleep(wait);

                Ok(Events::new(vec![TimeTaken(name.clone())], vec![]))
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
