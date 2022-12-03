use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use std::fmt::Debug;

pub const ALLOCATED: &'static str = "allocated";
pub const DEALLOCATED: &'static str = "deallocated";
const WORKER_STATE_NAME: &'static str = "test-worker";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WorkerCommand {
    Allocate(u32, ModelKey),
    Deallocate(u32, ModelKey),
}

impl Command for WorkerCommand {
    fn command_name(&self) -> &str {
        use WorkerCommand::*;
        match &self {
            Allocate(_, _) => "Allocate",
            Deallocate(_, _) => "Deallocate",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WorkerEvent {
    Allocated(u32),
    Deallocated(u32),
}

impl Event for WorkerEvent {
    fn event_name(&self) -> &str {
        use WorkerEvent::*;

        match &self {
            Allocated(_) => ALLOCATED,
            Deallocated(_) => DEALLOCATED,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WorkerNotification {
    Allocated(u32, ModelKey),
    Deallocated(u32, ModelKey),
}

impl Notification for WorkerNotification {
    fn notification_name(&self) -> &str {
        use WorkerNotification::*;

        match &self {
            Allocated(_, _) => ALLOCATED,
            Deallocated(_, _) => "deallocated",
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkerState {
    pub nb: u32,
    pub position: Option<u64>,
}

impl Default for WorkerState {
    fn default() -> Self {
        WorkerState {
            nb: 100,
            position: None,
        }
    }
}

impl State for WorkerState {
    type Event = WorkerEvent;
    type Command = WorkerCommand;
    type Notification = WorkerNotification;

    fn name_prefix() -> &'static str {
        WORKER_STATE_NAME
    }
    fn play_event(&mut self, event: &Self::Event) {
        use WorkerEvent::*;
        match event {
            Allocated(n) => self.nb -= n,
            Deallocated(n) => self.nb += n,
        }
    }

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>> {
        use WorkerCommand::*;

        match command {
            Allocate(n, k) => Ok(Events::new(
                vec![WorkerEvent::Allocated(*n)],
                vec![WorkerNotification::Allocated(*n, k.clone())],
            )),
            Deallocate(n, k) => Ok(Events::new(
                vec![WorkerEvent::Deallocated(*n)],
                vec![WorkerNotification::Deallocated(*n, k.clone())],
            )),
        }
    }

    fn get_position(&self) -> Option<u64> {
        self.position
    }

    fn set_position(&mut self, pos: Option<u64>) {
        self.position = pos;
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}
