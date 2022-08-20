mod command;
mod error;
mod event;
mod jwt_guard;
mod model;
mod query;

use crate::auth::command::handle_anonymous;
use crate::auth::event::AccountEvent;
use crate::auth::model::AccountModel;
use crate::auth::query::{account, register};
use api_account::AccountCommand;
use error::AccountError;
use eventstore::{Client, EventData, ReadStream};
use rocket::Route;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const STREAM_NAME: &str = "account";

pub fn get_route() -> Vec<Route> {
    routes![account, handle_anonymous, register]
}

async fn account_exist(db: &Client, id: String) -> Result<bool, AccountError> {
    let mut stream = get_stream(db, id).await?;

    Ok(stream.next().await.is_ok())
}

async fn load_account(db: &Client, id: String) -> Result<AccountModel, AccountError> {
    let mut stream = get_stream(db, id.clone()).await?;

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

async fn get_stream(db: &Client, id: String) -> Result<ReadStream, AccountError> {
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

async fn add_event(db: &Client, id: String, events: Vec<EventData>) -> Result<(), AccountError> {
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
                AccountCommand::Login(_) => "Login",
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

const JWT_SECRET: &str = "secret";
const JWT_ISSUER: &str = "royaumes-rs";
