use crate::multiple::build::BuildNotification;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use std::fmt::Debug;
use waiter::{CommandFromNotification, DeportedCommand};

pub const PAID: &'static str = "paid";
const GOLD_STATE_NAME: &'static str = "test-gold";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GoldCommand {
    Pay(u32, ModelKey),
}

impl Command for GoldCommand {
    fn command_name(&self) -> &str {
        use GoldCommand::*;
        match &self {
            Pay(_, _) => "Pay",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GoldEvent {
    Paid(u32),
}

impl Event for GoldEvent {
    fn event_name(&self) -> &str {
        use GoldEvent::*;

        match &self {
            Paid(_) => PAID,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum GoldNotification {
    Paid(u32, ModelKey),
}

impl Notification for GoldNotification {
    fn notification_name(&self) -> &str {
        use GoldNotification::*;

        match &self {
            Paid(_, _) => PAID,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GoldState {
    pub nb: u32,
    pub position: u64,
}

impl Default for GoldState {
    fn default() -> Self {
        GoldState {
            nb: 1000,
            position: 0,
        }
    }
}

impl State for GoldState {
    type Event = GoldEvent;
    type Command = GoldCommand;
    type Notification = GoldNotification;

    fn name_prefix() -> &'static str {
        GOLD_STATE_NAME
    }

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            GoldEvent::Paid(c) => self.nb -= c,
        }
    }

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>> {
        match command {
            GoldCommand::Pay(n, k) => Ok(Events::new(
                vec![GoldEvent::Paid(*n)],
                vec![GoldNotification::Paid(*n, k.clone())],
            )),
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

impl CommandFromNotification<BuildNotification, GoldCommand> for GoldCommand {
    fn get_command(
        notification: BuildNotification,
        notification_state_key: ModelKey,
    ) -> Option<DeportedCommand<GoldCommand>> {
        match notification {
            BuildNotification::AllocationNeeded(create) => Some(DeportedCommand {
                target_state_key: create.bank,
                command: GoldCommand::Pay(create.cost.gold, notification_state_key),
                duration: None,
            }),
            _ => None,
        }
    }
}
