use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use state_repository::ModelKey;
use tokio::time::Duration;
use waiter::CommandFromEvent;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SingleCommand {
    GrowStart(u32, u32),
    GrowEnd(u32),
    Add(u32),
}

pub const GROWTH_STARTED: &str = "growth_started";

impl Command for SingleCommand {
    fn name_prefix() -> &'static str {
        "wait"
    }

    fn command_name(&self) -> &str {
        use SingleCommand::*;
        match &self {
            GrowStart(_, _) => "GrowStart",
            GrowEnd(_) => "GrowEnd",
            Add(_) => "Add",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SingleEvent {
    Added(u32),
    Removed(u32),
    GrowthStarted(u32, u32),
    GrowthEnded(u32),
}

impl Event for SingleEvent {
    fn name_prefix() -> &'static str {
        "wait"
    }

    fn event_name(&self) -> &str {
        use SingleEvent::*;

        match &self {
            Added(_) => "added",
            Removed(_) => "removed",
            GrowthStarted(_, _) => GROWTH_STARTED,
            GrowthEnded(_) => "growth_ended",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SingleState {
    pub nb: u32,
    pub position: u64,
}

impl State for SingleState {
    type Event = SingleEvent;
    type Command = SingleCommand;

    fn play_event(&mut self, event: &Self::Event) {
        use SingleEvent::*;
        match event {
            Added(n) => self.nb += n,
            Removed(n) => self.nb -= n,
            GrowthStarted(n, _) => self.nb -= n,
            GrowthEnded(n) => self.nb += n,
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        use SingleCommand::*;
        use SingleEvent::*;
        match command {
            GrowStart(n, s) => {
                if *n > self.nb {
                    Err(anyhow!("{} cannot be grown to {}", n, self.nb))
                } else {
                    Ok(vec![GrowthStarted(*n, *s)])
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

impl CommandFromEvent<SingleEvent, SingleCommand> for SingleCommand {
    fn get_command(
        event: SingleEvent,
    ) -> Option<(SingleCommand, Option<ModelKey>, Option<Duration>)> {
        match event {
            SingleEvent::GrowthStarted(n, s) => Some((
                SingleCommand::GrowEnd(n * 2),
                None,
                Some(Duration::from_secs(s as u64)),
            )),
            _ => None,
        }
    }
}
