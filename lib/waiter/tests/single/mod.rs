use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use tokio::time::Duration;
use waiter::{DelayedCommand, DelayedCommandFromNotification};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SingleCommand {
    GrowStart(u32, u32),
    GrowEnd(u32),
    Add(u32),
}

pub const GROWTH_STARTED: &str = "growth_started";
const SINLGE_STATE_PREFIX: &'static str = "test-single";

impl Command for SingleCommand {
    fn command_name(&self) -> &'static str {
        use SingleCommand::*;
        match &self {
            GrowStart(_, _) => "GrowStart",
            GrowEnd(_) => "GrowEnd",
            Add(_) => "Add",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SingleEvent {
    Added(u32),
    Removed(u32),
    GrowthEnded(u32),
}

impl Event for SingleEvent {
    fn event_name(&self) -> &'static str {
        use SingleEvent::*;

        match &self {
            Added(_) => "added",
            Removed(_) => "removed",
            GrowthEnded(_) => "growth_ended",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SingleNotification {
    GrowthStarted(u32, u32),
}

impl Notification for SingleNotification {
    fn notification_name(&self) -> &str {
        use SingleNotification::*;

        match &self {
            GrowthStarted(_, _) => GROWTH_STARTED,
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SingleState {
    pub nb: u32,
    pub position: Option<u64>,
}

impl State for SingleState {
    type Event = SingleEvent;
    type Command = SingleCommand;
    type Notification = SingleNotification;

    fn name_prefix() -> &'static str {
        SINLGE_STATE_PREFIX
    }
    fn play_event(&mut self, event: &Self::Event) {
        use SingleEvent::*;
        match event {
            Added(n) => self.nb += n,
            Removed(n) => self.nb -= n,
            GrowthEnded(n) => self.nb += n,
        }
    }

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>> {
        use SingleCommand::*;
        use SingleEvent::*;
        use SingleNotification::*;
        match command {
            GrowStart(n, s) => {
                if *n > self.nb {
                    Err(anyhow!("{} cannot be grown to {}", n, self.nb))
                } else {
                    Ok(vec![Removed(*n)])
                }
            }
            GrowEnd(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![GrowthEnded(*n)])
                }
            }
            Add(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![Added(*n)])
                }
            }
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

impl DelayedCommandFromNotification<SingleNotification, SingleCommand> for SingleCommand {
    fn get_command(
        notification: SingleNotification,
        _notification_state_key: ModelKey,
    ) -> Option<DelayedCommand<SingleCommand>> {
        match notification {
            SingleNotification::GrowthStarted(n, s) => Some(DelayedCommand {
                command: SingleCommand::GrowEnd(n * 2),
                delay: Duration::from_secs(s as u64),
            }),
        }
    }
}
