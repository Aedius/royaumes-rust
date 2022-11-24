use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use tokio::time::Duration;
use waiter::WaitingState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WaitCommand {
    Grow(u32, u32),
    Add(u32),
}

impl Command for WaitCommand {
    fn name_prefix() -> &'static str {
        "wait"
    }

    fn command_name(&self) -> &str {
        match &self {
            WaitCommand::Grow(_, _) => "Grow",
            WaitCommand::Add(_) => "Add",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WaitEvent {
    Added(u32),
    Removed(u32),
    Wait(u32, u32),
    Noop,
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
            Wait(_, _) => "wait",
            Noop => "noop",
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
            Wait(_, _) => {}
            Noop => {}
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        match command {
            WaitCommand::Add(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![WaitEvent::Added(*n)])
                }
            }
            WaitCommand::Grow(n, s) => {
                if *n > self.nb {
                    Err(anyhow!("{} cannot be grown to {}", n, self.nb))
                } else {
                    Ok(vec![WaitEvent::Removed(*n), WaitEvent::Wait(*n * 2, *s)])
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

impl WaitingState for WaitState {
    fn get_next(event: &Self::Event) -> Option<(Self::Command, Duration)> {
        match event {
            WaitEvent::Wait(n, s) => Some((WaitCommand::Add(*n), Duration::from_secs(*s as u64))),
            _ => None,
        }
    }
}
