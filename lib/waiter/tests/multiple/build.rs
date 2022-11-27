use crate::multiple::gold::{ GoldNotification};
use crate::multiple::worker::{WorkerNotification};
use crate::multiple::Cost;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, Event, Events, Notification, State};
use state_repository::ModelKey;
use std::time::Duration;
use waiter::{CommandFromNotification, DeportedCommand};

pub const ALLOCATION_NEEDED: &'static str = "allocation_needed";
pub const BUILD_ENDED: &'static str = "build_ended";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BuildingCreate {
    pub cost: Cost,
    pub bank: ModelKey,
    pub citizen: ModelKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildCommand {
    Create(BuildingCreate),
    Build(Duration),
    Allocate(Cost),
    FinnishBuild,
}

impl Command for BuildCommand {
    fn name_prefix() -> &'static str {
        "build"
    }

    fn command_name(&self) -> &str {
        use BuildCommand::*;
        match &self {
            Create(_) => "Create",
            Allocate(_) => "Allocate",
            Build(_) => "Build",
            FinnishBuild => "FinnishBuild",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BuildEvent {
    Created(BuildingCreate),
    Allocated(Cost),
    BuildStarted(Duration),
    Built,
}

impl Event for BuildEvent {
    fn name_prefix() -> &'static str {
        "build"
    }

    fn event_name(&self) -> &str {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            Allocated(_) => "allocated",
            BuildStarted(_) => "build_started",
            Built => "build",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BuildNotification{
    AllocationNeeded(BuildingCreate),
    BuildEnded(),
}

impl Notification for BuildNotification{
    fn name_prefix() -> &'static str {
        "build"
    }

    fn notification_name(&self) -> &str {
        use BuildNotification::*;

        match &self {
            AllocationNeeded(_) => ALLOCATION_NEEDED,
            BuildEnded() => BUILD_ENDED,
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
    type Notification =BuildNotification;

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
            BuildStarted(_) => {}
            Built => {
                self.built = true
            }
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
            Create(c) => Ok(Events::new(vec![Created(c.clone())], vec![AllocationNeeded(c.clone())])),
            Allocate(c) => Ok(Events::new(vec![Allocated(*c)], vec![])),
            Build(d) => Ok(Events::new(vec![BuildStarted(*d)], vec![])),
            FinnishBuild => Ok(Events::new(vec![Built], vec![BuildEnded()])),
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
        event: GoldNotification,
        _state_key: ModelKey,
    ) -> Option<DeportedCommand<BuildCommand>> {
        match event {
            GoldNotification::Paid(gold, paid_key) => Some(DeportedCommand {
                command: BuildCommand::Allocate(Cost { gold, worker: 0 }),
                key: paid_key,
                duration: None,
            }),
        }
    }
}

impl CommandFromNotification<WorkerNotification, BuildCommand> for BuildCommand {
    fn get_command(
        event: WorkerNotification,
        _state_key: ModelKey,
    ) -> Option<DeportedCommand<BuildCommand>> {
        match event {
            WorkerNotification::Allocated(worker, paid_key) => Some(DeportedCommand {
                command: BuildCommand::Allocate(Cost { gold: 0, worker }),
                key: paid_key,
                duration: None,
            }),
            _ => None,
        }
    }
}
