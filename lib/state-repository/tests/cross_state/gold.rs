use crate::cross_state::flow::{Payment, PaymentPaid};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use state_repository::model_key::ModelKey;
use std::fmt::Debug;

pub const PAID: &'static str = "paid";
pub const GOLD_STATE_NAME: &'static str = "test-gold";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GoldCommand {
    Pay(u32, ModelKey),
}

impl Command for GoldCommand {
    fn command_name(&self) -> &'static str {
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
    fn event_name(&self) -> &'static str {
        use GoldEvent::*;

        match &self {
            Paid(_) => PAID,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GoldState {
    pub nb: u32,
    pub position: Option<u64>,
}

impl Default for GoldState {
    fn default() -> Self {
        GoldState {
            nb: 1000,
            position: None,
        }
    }
}

impl State for GoldState {
    type Event = GoldEvent;
    type Command = GoldCommand;

    fn name_prefix() -> &'static str {
        GOLD_STATE_NAME
    }

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            GoldEvent::Paid(c) => self.nb -= c,
        }
    }

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            GoldCommand::Pay(n, k) => Ok(vec![GoldEvent::Paid(*n)]),
        }
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}

// impl Distant<Payment> for GoldState {
//     fn get_command(input: <Payment as Flow>::Input) -> Self::Command {
//         GoldCommand::Pay(input.amount, input.target)
//     }
//
//     fn get_response(output: Self::Notification) -> <Payment as Flow>::Output {
//         match output {
//             GoldNotification::Paid(amount, target) => PaymentPaid::Paid(amount, target),
//         }
//     }
//
//     fn get_notification_response() -> Vec<&'static str> {
//         vec![PAID]
//     }
// }
