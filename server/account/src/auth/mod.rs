mod command;
mod error;
mod event;
mod model;
mod query;

use crate::auth::command::{handle, AccountCommand};
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

async fn add_event(db: &Client, id: &Id, events: Vec<Account>) -> Result<(), AccountError> {
    let events_data: Vec<EventData> = events.into_iter().map(|e| e.to_event_data()).collect();

    let added = db
        .append_to_stream(
            format!("{}-{}", STREAM_NAME, id),
            &Default::default(),
            events_data,
        )
        .await;

    match added {
        Ok(_) => Ok(()),
        Err(err) => Err(AccountError::Other(format!("Cannot add event : {:?}", err))),
    }
}

use crate::auth::model::AccountModel;
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
pub enum Account {
    Event(AccountEvent),
    Command(AccountCommand),
    Error(AccountError),
}

impl Account {
    pub fn to_event_data(&self) -> EventData {
        match self {
            Account::Event(event) => match event {
                AccountEvent::Created(_) => EventData::json("AccountCreated", &self).unwrap(),
                AccountEvent::Added(_) => EventData::json("QuantityAdded", &self).unwrap(),
                AccountEvent::Removed(_) => EventData::json("QuantityRemoved", &self).unwrap(),
            },
            Account::Command(command) => match command {
                AccountCommand::CreateAccount(_) => {
                    EventData::json("CreateAccount", &self).unwrap()
                }
                AccountCommand::AddQuantity(_) => EventData::json("AddQuantity", &self).unwrap(),
                AccountCommand::RemoveQuantity(_) => {
                    EventData::json("RemoveQuantity", &self).unwrap()
                }
            },
            Account::Error(error) => match error {
                AccountError::NotFound(_) => {
                    EventData::json("ErrorAccountNotFound", &self).unwrap()
                }
                AccountError::AlreadyExist(_) => {
                    EventData::json("ErrorAccountAlreadyExist", &self).unwrap()
                }
                AccountError::WrongQuantity(_) => {
                    EventData::json("ErrorAccountWrongQuantity", &self).unwrap()
                }
                AccountError::Other(_) => EventData::json("ErrorAccountOther", &self).unwrap(),
            },
        }
    }
}
