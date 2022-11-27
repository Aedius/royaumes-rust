use crate::multiple::Cost;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WorkerCommand {
    Allocate(Cost),
    Deallocate(Cost),
}

impl Command for WorkerCommand {
    fn name_prefix() -> &'static str {
        "worker"
    }

    fn command_name(&self) -> &str {
        use WorkerCommand::*;
        match &self {
            Allocate(_) => "Allocate",
            Deallocate(_) => "Deallocate",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WorkerEvent {
    Allocated(Cost),
    Deallocated(Cost),
}

impl Event for WorkerEvent {
    fn name_prefix() -> &'static str {
        "worker"
    }

    fn event_name(&self) -> &str {
        use WorkerEvent::*;

        match &self {
            Allocated(_) => "allocated",
            Deallocated(_) => "deallocated",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkerState {
    nb: u32,
    pub position: u64,
}

impl Default for WorkerState {
    fn default() -> Self {
        WorkerState {
            nb: 100,
            position: 0,
        }
    }
}

impl State for WorkerState {
    type Event = WorkerEvent;
    type Command = WorkerCommand;

    fn play_event(&mut self, event: &Self::Event) {
        use WorkerEvent::*;
        match event {
            Allocated(c) => self.nb -= c.worker,
            Deallocated(c) => self.nb += c.worker,
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        use WorkerCommand::*;
        use WorkerEvent::*;

        match command {
            Allocate(n) => Ok(vec![Allocated(*n)]),
            Deallocate(n) => Ok(vec![Deallocated(*n)]),
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
