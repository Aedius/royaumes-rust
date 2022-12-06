use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, CommandName, Event, EventName, State, StateName};
use state_repository::waiter::DelayedState;
use tokio::time::Duration;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WaitCommand {
    Growth(u32),
    GrowEnd(u32),
    Add(u32),
}

pub const GROWTH_STARTED: &str = "growth_started";
const SINLGE_STATE_PREFIX: &'static str = "test-wait";

impl Command for WaitCommand {
    fn command_name(&self) -> CommandName {
        use WaitCommand::*;
        match &self {
            Growth(_) => "Growth",
            GrowEnd(_) => "GrowEnd",
            Add(_) => "Add",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WaitEvent {
    Added(u32),
    Removed(u32),
    GrowthStarted(u32, Duration),
}

impl Event for WaitEvent {
    fn event_name(&self) -> EventName {
        use WaitEvent::*;

        match &self {
            Added(_) => "added",
            Removed(_) => "removed",
            GrowthStarted(_, _) => GROWTH_STARTED,
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct WaitState {
    pub nb: u32,
}

impl State for WaitState {
    type Event = WaitEvent;
    type Command = WaitCommand;

    fn name_prefix() -> StateName {
        SINLGE_STATE_PREFIX
    }
    fn play_event(&mut self, event: &Self::Event) {
        use WaitEvent::*;
        match event {
            Added(n) => self.nb += n,
            Removed(n) => self.nb -= n,
            GrowthStarted(_, _) => {}
        }
    }

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>> {
        use WaitCommand::*;
        use WaitEvent::*;
        match command {
            GrowEnd(n) => {
                if self.nb.checked_add(n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![Added(n)])
                }
            }
            Add(n) => {
                if self.nb.checked_add(n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(vec![Added(n)])
                }
            }
            Growth(n) => Ok(vec![Removed(n), GrowthStarted(n, Duration::from_secs(2))]),
        }
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}

impl DelayedState for WaitState {
    fn event_to_delayed() -> Vec<EventName> {
        vec![GROWTH_STARTED]
    }

    fn resolve_command(event: Self::Event) -> (Self::Command, Duration) {
        use WaitCommand::*;
        use WaitEvent::*;
        match event {
            GrowthStarted(n, d) => (GrowEnd(n * 2), d),
            _ => {
                unimplemented!()
            }
        }
    }
}
