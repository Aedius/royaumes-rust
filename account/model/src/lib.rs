use crate::error::AccountError;
use crate::event::AccountEvent;
use account_api::AccountCommand;
use eventstore::EventData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;
pub mod event;
pub mod model;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Metadata {
    #[serde(rename = "$correlationId")]
    correlation_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Account {
    Event(AccountEvent),
    Command(AccountCommand),
    Error(AccountError),
}

impl Account {
    pub fn event_name(&self) -> &str {
        match self {
            Account::Event(event) => match event {
                AccountEvent::Created(_) => "AccountCreated",
                AccountEvent::AccountAdded(_) => "QuantityAdded",
                AccountEvent::AccountRemoved(_) => "QuantityRemoved",
                AccountEvent::Logged(_) => "Logged",
                AccountEvent::Joined(_) => "JoinedServer",
                AccountEvent::Leaved(_) => "LeavedServer",
            },
            Account::Command(command) => match command {
                AccountCommand::CreateAccount(_) => "CreateAccount",
                AccountCommand::AddQuantity(_) => "AddQuantity",
                AccountCommand::RemoveQuantity(_) => "RemoveQuantity",
                AccountCommand::Login(_) => "Login",
                AccountCommand::Join(_) => "JoinServer",
                AccountCommand::Leave(_) => "LeaveServer",
            },
            Account::Error(error) => match error {
                AccountError::NotFound(_) => "ErrorAccountNotFound",
                AccountError::AlreadyExist(_) => "ErrorAccountAlreadyExist",
                AccountError::WrongQuantity(_) => "ErrorAccountWrongQuantity",
                AccountError::Other(_) => "ErrorAccountOther",
            },
        }
    }

    pub fn to_event_data(&self, previous: Option<Uuid>) -> (EventData, Uuid) {
        let id = Uuid::new_v4();
        let mut event_data = EventData::json(self.event_name(), self).unwrap();
        event_data = event_data.id(id);

        if let Some(previous_uuid) = previous {
            event_data = event_data
                .metadata_as_json(Metadata {
                    correlation_id: previous_uuid,
                })
                .unwrap();
        }

        (event_data, id)
    }
}
