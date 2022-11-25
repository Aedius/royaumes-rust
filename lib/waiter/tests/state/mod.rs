use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use tokio::time::Duration;
use waiter::WaitingState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WaitCommand {
    GrowStart(u32, u32),
    GrowEnd(u32),
    Add(u32),
}

impl Command for WaitCommand {
    fn name_prefix() -> &'static str {
        "wait"
    }

    fn command_name(&self) -> &str {
        use WaitCommand::*;
        match &self {
            GrowStart(_, _) => "GrowStart",
            GrowEnd(_) => "GrowEnd",
            Add(_) => "Add",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WaitEvent {
    Added(u32),
    Removed(u32),
    GrowthStarted(u32, u32),
    GrowthEnded(u32),
}

impl Event for WaitEvent {
    fn name_prefix() -> &'static str {
        "wait"
    }

    fn event_name(&self) -> &str {
        use WaitEvent::*;

        match &self {
            Added(_) => "added",
            Removed(_) => "removed",
            GrowthStarted(_, _) => "growth_started",
            GrowthEnded(_) => "growth_ended",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WaitState {
    pub nb: u32,
    pub position: u64,
}

impl State for WaitState {
    type Event = WaitEvent;
    type Command = WaitCommand;

    fn play_event(&mut self, event: &Self::Event) {
        use WaitEvent::*;
        match event {
            Added(n) => self.nb += n,
            Removed(n) => self.nb -= n,
            GrowthStarted(n, _) => self.nb -= n,
            GrowthEnded(n) => self.nb += n,
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        match command {
            WaitCommand::GrowStart(n, s) => {
                if *n > self.nb {
                    Err(anyhow!("{} cannot be grown to {}", n, self.nb))
                } else {
                    Ok(vec![WaitEvent::GrowthStarted(*n, *s)])
                }
            }
            WaitCommand::GrowEnd(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![WaitEvent::GrowthEnded(*n)])
                }
            }
            WaitCommand::Add(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![WaitEvent::Added(*n)])
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

impl WaitingState<WaitCommand> for WaitState {
    fn get_next(event: &Self::Event) -> Option<(Self::Command, Duration)> {
        match event {
            WaitEvent::GrowthStarted(n, s) => {
                Some((WaitCommand::GrowEnd(*n * 2), Duration::from_secs(*s as u64)))
            }
            _ => None,
        }
    }
}
