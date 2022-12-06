use anyhow::Result;
use serde::{Deserialize, Serialize};
use state::{Command, CommandName, Event, EventName, State};
use state_repository::cross_state::{CrossData, CrossDataProcessor, CrossStateAnswer};
use state_repository::model_key::ModelKey;
use eventstore::RecordedEvent;
use crate::cross_state::build_api::{PaymentQuestion, PublicBuild};

pub const BUILD_STATE_NAME: &'static str = "test-tower";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BuildingCreate {
    pub cost: u32,
    pub bank: ModelKey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BuildCommand {
    Create(BuildingCreate),
    Pay(u32),
}

impl Command for BuildCommand {
    fn command_name(&self) -> CommandName {
        use BuildCommand::*;
        match &self {
            Create(_) => "Create",
            Pay(_) => "Pay",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum BuildEvent {
    Created(BuildingCreate),
    Built,
    Public(PaymentQuestion),
}

impl Event for BuildEvent {
    fn event_name(&self) -> EventName {
        use BuildEvent::*;
        match &self {
            Created(_) => "created",
            Built => "build",
            Public(p) => p.event_name(),
        }
    }

    fn is_state_specific(&self) -> bool {
        match &self {
            BuildEvent::Public(_) => false,
            _ => true
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildState {
    pub cost: u32,
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
            Built => self.built = true,
            Public(_) => {}
        }
    }

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>> {
        use BuildCommand::*;
        use BuildEvent::*;
        match command {
            Create(c) => Ok(vec![
                Created(c.clone()),
                Public(PaymentQuestion {
                    amount: c.cost,
                    bank: c.bank.clone(),
                }),
            ]),
            Pay(c) => {
                if c >= self.cost {
                    Ok(vec![Built])
                } else {
                    todo!("in real case it should be handle wink")
                }
            }
        }
    }

    fn state_cache_interval() -> Option<u64> {
        None
    }
}

impl CrossDataProcessor for BuildState {
    fn resolve(e: RecordedEvent, _local_key: ModelKey) -> (Self::Command, ModelKey) {
        Self::resolve_helper(e)
    }
}

impl CrossStateAnswer<PublicBuild> for BuildState{
    fn resolve_answer(event: <PublicBuild as CrossData>::Answer) -> Self::Command {
        BuildCommand::Pay(event.amount)
    }
}
