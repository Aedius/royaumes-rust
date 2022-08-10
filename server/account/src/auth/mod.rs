mod command;
mod error;
mod event;
mod model;
mod query;

use crate::auth::command::handle;
use crate::auth::event::AccountEvent;
use crate::auth::query::{get, register};
use error::AccountError;
use eventstore::{Client, EventData, ReadStream};
use rocket::Route;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;
const STREAM_NAME: &str = "account";

pub fn get_route() -> Vec<Route> {
    routes![get, handle, register]
}

async fn account_exist(db: &Client, id: &Id) -> Result<bool, AccountError> {
    let mut stream = get_stream(db, id).await?;

    Ok(stream.next().await.is_ok())
}

async fn load_account(db: &Client, id: &Id) -> Result<AccountModel, AccountError> {
    let mut stream = get_stream(db, id).await?;

    let mut account = AccountModel::default();
    let mut exist = false;
    // region iterate-stream
    while let Ok(Some(event)) = stream.next().await {
        exist = true;

        let account_event = event.get_original_event().as_json::<Account>();

        match account_event {
            Ok(account_event) => {
                account.play_event(account_event);
            }
            Err(err) => {
                warn!("Unable to json decode : {:?}, got error {:?}", event, err);
            }
        }
    }

    if exist {
        Ok(account)
    } else {
        Err(AccountError::NotFound(format!(
            "account `{}` not found",
            id
        )))
    }
}

async fn get_stream(db: &Client, id: &Id) -> Result<ReadStream, AccountError> {
    let res = db
        .read_stream(format!("{}-{}", STREAM_NAME, id), &Default::default())
        .await;

    let stream = match res {
        Ok(s) => s,
        Err(err) => {
            return Err(AccountError::Other(format!(
                "Cannot connect to evenstore : {:?}",
                err
            )))
        }
    };
    Ok(stream)
}

async fn add_event(db: &Client, id: &Id, events: Vec<EventData>) -> Result<(), AccountError> {
    let added = db
        .append_to_stream(
            format!("{}-{}", STREAM_NAME, id),
            &Default::default(),
            events,
        )
        .await;

    match added {
        Ok(_) => Ok(()),
        Err(err) => Err(AccountError::Other(format!("Cannot add event : {:?}", err))),
    }
}

use crate::auth::model::AccountModel;
use api_account::AccountCommand;
use rocket::request::FromParam;

pub struct Id {
    uuid: String,
}

impl From<Uuid> for Id {
    fn from(uuid: Uuid) -> Self {
        Id {
            uuid: uuid.to_string(),
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl<'r> FromParam<'r> for Id {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param) {
            Ok(uuid) => Ok(Id::from(uuid)),
            Err(_) => Err(param),
        }
    }
}

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
                AccountEvent::Added(_) => "QuantityAdded",
                AccountEvent::Removed(_) => "QuantityRemoved",
            },
            Account::Command(command) => match command {
                AccountCommand::CreateAccount(_) => "CreateAccount",
                AccountCommand::AddQuantity(_) => "AddQuantity",
                AccountCommand::RemoveQuantity(_) => "RemoveQuantity",
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
        let mut event_data = EventData::json(self.event_name(), &self).unwrap();
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
