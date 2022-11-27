use crate::multiple::Cost;
use serde::{Deserialize, Serialize};
use state::{Command, Event, State};
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildCommand {
    Create(Cost),
    Allocate(Cost),
    Build(Duration),
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

#[derive(Debug, Deserialize, Serialize)]
pub enum BuildEvent {
    Created(Cost),
    AllocationNeeded(Cost),
    Allocated(Cost),
    BuildStarted(Duration),
    BuildEnded(),
}

impl Event for BuildEvent {
    fn name_prefix() -> &'static str {
        "build"
    }

    fn event_name(&self) -> &str {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            AllocationNeeded(_) => "allocation_needed",
            Allocated(_) => "allocated",
            BuildStarted(_) => "build_started",
            BuildEnded() => "build_ended",
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BuildState {
    pub cost: Cost,
    pub allocated: Cost,
    pub built: bool,
    pub position: u64,
}

impl State for BuildState {
    type Event = BuildEvent;
    type Command = BuildCommand;

    fn play_event(&mut self, event: &Self::Event) {
        use BuildEvent::*;
        match event {
            Created(cost) => {
                self.cost = *cost;
            }
            AllocationNeeded(_cost) => {}
            Allocated(allocated) => {
                self.cost = Cost {
                    gold: self.cost.gold + allocated.gold,
                    worker: self.cost.worker + allocated.worker,
                }
            }
            BuildStarted(_) => {}
            BuildEnded() => self.built = true,
        }
    }

    fn try_command(&self, command: &Self::Command) -> anyhow::Result<Vec<Self::Event>> {
        use BuildCommand::*;
        use BuildEvent::*;
        match command {
            Create(c) => Ok(vec![Created(*c)]),
            Allocate(c) => Ok(vec![AllocationNeeded(*c)]),
            Build(d) => Ok(vec![BuildStarted(*d)]),
            FinnishBuild => Ok(vec![BuildEnded()]),
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
