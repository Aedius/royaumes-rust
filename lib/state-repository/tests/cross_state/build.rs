use crate::cross_state::flow::{AskPayment, CrossPayment, PaymentNeeded, PaymentPaid, PaymentPay};
use crate::cross_state::Cost;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use state_repository::model_key::ModelKey;
use std::time::Duration;
use state_repository::cross_state::{CrossStateAnswer, CrossStateQuestion};

pub const ALLOCATION_NEEDED: &'static str = "allocation_needed";
pub const BUILD_ENDED: &'static str = "build_ended";
pub const BUILD_STARTED: &'static str = "build_started";
pub const BUILD_STATE_NAME: &'static str = "test-tower";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BuildingCreate {
    pub cost: u32,
    pub bank: ModelKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildCommand {
    Create(BuildingCreate),
    Allocate(u32),
}

impl Command for BuildCommand {
    fn command_name(&self) -> &'static str {
        use BuildCommand::*;
        match &self {
            Create(_) => "Create",
            Allocate(_) => "Allocate",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BuildEvent {
    Created(BuildingCreate),
    Allocated(u32),
    Built,
}

impl Event for BuildEvent {
    fn event_name(&self) -> &'static str {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            Allocated(_) => "allocated",
            Built => "build",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildState {
    pub cost: u32,
    pub allocated: u32,
    pub built: bool,
    pub bank: Option<ModelKey>,
}

impl State for BuildState {
    type Event = BuildEvent;
    type Command = BuildCommand;

    fn name_prefix() -> &'static str {
        BUILD_STATE_NAME
    }

    fn play_event(&mut self, event: &Self::Event) {
        use BuildEvent::*;

        match event {
            Created(create) => {
                self.cost = create.cost;
                self.bank = Some(create.bank.clone());
            }
            Allocated(allocated) => {
                self.allocated += allocated
            }
            Built => self.built = true,
        }
    }

    fn try_command(&self, command: &Self::Command) -> Result<Vec<Self::Event>> {
        use BuildCommand::*;
        use BuildEvent::*;
        use BuildNotification::*;
        match command {
            Create(c) => Ok(vec![Created(c.clone())]),
            Allocate(c) => {
                Ok(vec![Allocated(*c)])
            }
        }
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}

impl CrossStateAnswer<CrossPayment> for BuildState{
    fn resolve_answer(event: CrossPayment::Answer) -> (Self::Command, ModelKey) {
        todo!()
    }
}


// impl Distant<AskPayment> for BuildState {
//     fn get_command(input: <AskPayment as Flow>::Input) -> Self::Command {
//         match input {
//             PaymentPaid::Paid(amount, _) => BuildCommand::Allocate(Cost {
//                 gold: amount,
//                 worker: 0,
//             }),
//             PaymentPaid::NotPaid(_) => BuildCommand::Allocate(Cost { gold: 0, worker: 0 }),
//         }
//     }
//
//     fn get_response(output: Self::Notification) -> <AskPayment as Flow>::Output {
//         match output {
//             BuildNotification::AllocationNeeded(c) => PaymentPay {
//                 amount: c.cost.gold,
//                 target: c.bank,
//             },
//             _ => {
//                 panic!("not implemented");
//             }
//         }
//     }
//
//     fn get_notification_response() -> Vec<&'static str> {
//         vec![ALLOCATION_NEEDED]
//     }
// }
