use crate::multiple::gold::GoldNotification;
use crate::multiple::worker::WorkerNotification;
use crate::multiple::Cost;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use std::time::Duration;
use waiter::{CommandFromNotification, DeportedCommand};

pub const ALLOCATION_NEEDED: &'static str = "allocation_needed";
pub const BUILD_ENDED: &'static str = "build_ended";
pub const BUILD_STARTED: &'static str = "build_started";
const BUILD_STATE_NAME: &'static str = "test-build";

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    fn command_name(&self) -> &str {
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
    fn event_name(&self) -> &str {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            Allocated(_) => "allocated",
            Deallocated(_) => "deallocated",
            Built => "build",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub position: u64,
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

    fn try_command(
        &self,
        command: &Self::Command,
    ) -> Result<Events<Self::Event, Self::Notification>> {
        use BuildCommand::*;
        use BuildEvent::*;
        use BuildNotification::*;
        match command {
            Create(c) => Ok(Events::new(
                vec![Created(c.clone())],
                vec![AllocationNeeded(c.clone())],
            )),
            Allocate(c) => {
                let mut notifications = Vec::new();

                let mut total = self.allocated;
                total.worker += c.worker;
                total.gold += c.gold;

                if total.worker >= self.cost.worker && total.gold >= self.cost.worker {
                    notifications.push(BuildStarted(Duration::from_secs(2)));
                }

                Ok(Events::new(vec![Allocated(*c)], notifications))
            }
            Deallocate(c) => Ok(Events::new(vec![Deallocated(*c)], Vec::new())),
            FinnishBuild => Ok(Events::new(
                vec![Built],
                vec![BuildEnded(BuildingCreate {
                    cost: self.cost,
                    bank: self.bank.clone().unwrap(),
                    citizen: self.citizen.clone().unwrap(),
                })],
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

impl CommandFromNotification<GoldNotification, BuildCommand> for BuildCommand {
    fn get_command(
        notification: GoldNotification,
        _notification_state_key: ModelKey,
    ) -> Option<DeportedCommand<BuildCommand>> {
        match notification {
            GoldNotification::Paid(gold, paid_key) => Some(DeportedCommand {
                command: BuildCommand::Allocate(Cost { gold, worker: 0 }),
                target_state_key: paid_key,
                duration: None,
            }),
        }
    }
}

impl CommandFromNotification<WorkerNotification, BuildCommand> for BuildCommand {
    fn get_command(
        notification: WorkerNotification,
        _notification_state_key: ModelKey,
    ) -> Option<DeportedCommand<BuildCommand>> {
        match notification {
            WorkerNotification::Allocated(worker, paid_key) => Some(DeportedCommand {
                command: BuildCommand::Allocate(Cost { gold: 0, worker }),
                target_state_key: paid_key,
                duration: None,
            }),
            WorkerNotification::Deallocated(worker, paid_key) => Some(DeportedCommand {
                command: BuildCommand::Deallocate(Cost { gold: 0, worker }),
                target_state_key: paid_key,
                duration: None,
            }),
        }
    }
}

impl CommandFromNotification<BuildNotification, BuildCommand> for BuildCommand {
    fn get_command(
        notification: BuildNotification,
        notification_state_key: ModelKey,
    ) -> Option<DeportedCommand<BuildCommand>> {
        match notification {
            BuildNotification::BuildStarted(s) => Some(DeportedCommand {
                command: BuildCommand::FinnishBuild,
                target_state_key: notification_state_key,
                duration: Some(s),
            }),
            _ => None,
        }
    }
}
