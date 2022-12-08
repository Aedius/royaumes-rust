use crate::cross_state::build_api::{PaymentResponse, PublicBuild};
use anyhow::Result;
use eventstore::RecordedEvent;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use state_repository::cross_state::{CrossData, CrossDataProcessor, CrossStateQuestion};
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
pub struct Cost {
    amount: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum GoldEvent {
    Paid(Cost),
    Public(PaymentResponse),
}

impl Event for GoldEvent {
    fn event_name(&self) -> &'static str {
        use GoldEvent::*;

        match &self {
            Paid(_) => PAID,
            Public(p) => p.event_name(),
        }
    }

    fn is_state_specific(&self) -> bool {
        match &self {
            GoldEvent::Public(_) => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GoldState {
    pub nb: u32,
}

impl Default for GoldState {
    fn default() -> Self {
        GoldState { nb: 1000 }
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
            GoldEvent::Paid(c) => self.nb -= c.amount,
            GoldEvent::Public(_) => {}
        }
    }

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>> {
        match command {
            GoldCommand::Pay(n, k) => Ok(vec![
                GoldEvent::Paid(Cost { amount: n }),
                GoldEvent::Public(PaymentResponse {
                    amount: n,
                    response: k,
                }),
            ]),
        }
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}

impl CrossDataProcessor for GoldState {
    fn resolve(e: RecordedEvent, local_key: ModelKey) -> (Self::Command, ModelKey) {
        Self::resolve_helper(e, local_key)
    }
}

impl CrossStateQuestion<PublicBuild> for GoldState {
    fn resolve_question(
        event: <PublicBuild as CrossData>::Question,
        local_key: ModelKey,
    ) -> Self::Command {
        GoldCommand::Pay(event.amount, local_key)
    }
}
