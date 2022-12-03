use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, State};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum SimpleCommand {
    Add(u32),
    Remove(u32),
    Set(u32),
}

impl Command for SimpleCommand {
    fn command_name(&self) -> &str {
        match &self {
            SimpleCommand::Add(_) => "Add",
            SimpleCommand::Remove(_) => "Remove",
            SimpleCommand::Set(_) => "Set",
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SimpleEvent {
    Added(u32),
    Removed(u32),
}

impl Event for SimpleEvent {
    fn event_name(&self) -> &str {
        match &self {
            SimpleEvent::Added(_) => "added",
            SimpleEvent::Removed(_) => "removed",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SimpleState {
    pub nb: u32,
    pub position: Option<u64>,
}

impl State for SimpleState {
    type Event = SimpleEvent;
    type Command = SimpleCommand;
    type Notification = ();

    fn name_prefix() -> &'static str {
        "test-simple"
    }
    fn play_event(&mut self, event: &Self::Event) {
        match event {
            SimpleEvent::Added(n) => self.nb += n,
            SimpleEvent::Removed(n) => self.nb -= n,
        }
    }

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>> {
        match command {
            SimpleCommand::Add(n) => {
                if self.nb.checked_add(*n).is_none() {
                    Err(anyhow!("{} cannot be added to {}", n, self.nb))
                } else {
                    Ok(Events::new(vec![SimpleEvent::Added(*n)], vec![]))
                }
            }
            SimpleCommand::Remove(n) => {
                if *n > self.nb {
                    Err(anyhow!("{} cannot be removed to {}", n, self.nb))
                } else {
                    Ok(Events::new(vec![SimpleEvent::Removed(*n)], vec![]))
                }
            }
            SimpleCommand::Set(n) => Ok(Events::new(
                vec![SimpleEvent::Removed(self.nb), SimpleEvent::Added(*n)],
                vec![],
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
        Some(1)
    }
}
