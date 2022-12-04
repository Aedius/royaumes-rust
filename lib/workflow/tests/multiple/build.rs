use crate::multiple::flow::{AskPayment, PaymentPaid, PaymentPay};
use crate::multiple::Cost;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use std::time::Duration;
use workflow::{Distant, Flow};

pub const ALLOCATION_NEEDED: &'static str = "allocation_needed";
pub const BUILD_ENDED: &'static str = "build_ended";
pub const BUILD_STARTED: &'static str = "build_started";
const BUILD_STATE_NAME: &'static str = "test-tower";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BuildingCreate {
    pub cost: Cost,
    pub bank: ModelKey,
    pub citizen: ModelKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildCommand {
    Create(BuildingCreate),
    Allocate(Cost),
    Deallocate(Cost),
    FinnishBuild,
}

impl Command for BuildCommand {
    fn command_name(&self) -> &'static str {
        use BuildCommand::*;
        match &self {
            Create(_) => "Create",
            Allocate(_) => "Allocate",
            Deallocate(_) => "Deallocate",
            FinnishBuild => "FinnishBuild",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BuildEvent {
    Created(BuildingCreate),
    Allocated(Cost),
    Deallocated(Cost),
    Built,
}

impl Event for BuildEvent {
    fn event_name(&self) -> &'static str {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            Allocated(_) => "allocated",
            Deallocated(_) => "deallocated",
            Built => "build",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum BuildNotification {
    AllocationNeeded(BuildingCreate),
    BuildStarted(Duration),
    BuildEnded(BuildingCreate),
}

impl Notification for BuildNotification {
    fn notification_name(&self) -> &str {
        use BuildNotification::*;

        match &self {
            AllocationNeeded(_) => ALLOCATION_NEEDED,
            BuildStarted(_) => BUILD_STARTED,
            BuildEnded(_) => BUILD_ENDED,
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildState {
    pub cost: Cost,
    pub allocated: Cost,
    pub built: bool,
    pub citizen: Option<ModelKey>,
    pub bank: Option<ModelKey>,
    pub position: Option<u64>,
}

impl State for BuildState {
    type Event = BuildEvent;
    type Command = BuildCommand;
    type Notification = BuildNotification;

    fn name_prefix() -> &'static str {
        BUILD_STATE_NAME
    }

    fn play_event(&mut self, event: &Self::Event) {
        use BuildEvent::*;

        match event {
            Created(create) => {
                self.cost = create.cost.clone();
                self.citizen = Some(create.citizen.clone());
                self.bank = Some(create.bank.clone());
            }
            Allocated(allocated) => {
                self.allocated = Cost {
                    gold: self.allocated.gold + allocated.gold,
                    worker: self.allocated.worker + allocated.worker,
                }
            }
            Deallocated(deallocated) => {
                self.allocated = Cost {
                    gold: self.allocated.gold - deallocated.gold,
                    worker: self.allocated.worker - deallocated.worker,
                }
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
                let mut notifications = Vec::new();

                let mut total = self.allocated;
                total.worker += c.worker;
                total.gold += c.gold;

                if total.worker >= self.cost.worker && total.gold >= self.cost.worker {
                    notifications.push(BuildStarted(Duration::from_secs(2)));
                }

                Ok(vec![Allocated(*c)])
            }
            Deallocate(c) => Ok(vec![Deallocated(*c)]),
            FinnishBuild => Ok(vec![Built]),
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

impl Distant<AskPayment> for BuildState {
    fn get_command(input: <AskPayment as Flow>::Input) -> Self::Command {
        match input {
            PaymentPaid::Paid(amount, _) => BuildCommand::Allocate(Cost {
                gold: amount,
                worker: 0,
            }),
            PaymentPaid::NotPaid(_) => BuildCommand::Allocate(Cost { gold: 0, worker: 0 }),
        }
    }

    fn get_response(output: Self::Notification) -> <AskPayment as Flow>::Output {
        match output {
            BuildNotification::AllocationNeeded(c) => PaymentPay {
                amount: c.cost.gold,
                target: c.bank,
            },
            _ => {
                panic!("not implemented");
            }
        }
    }

    fn get_notification_response() -> Vec<&'static str> {
        vec![ALLOCATION_NEEDED]
    }
}
