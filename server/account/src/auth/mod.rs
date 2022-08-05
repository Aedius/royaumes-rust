mod command;
mod error;
mod event;
mod model;
mod query;

use crate::auth::command::{add, create, remove};
use crate::auth::event::AccountEvent;
use crate::auth::model::Account;
use crate::auth::query::{get, register};
use error::AuthError;
use eventstore::{Client, ReadStream};
use rocket::Route;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

const STREAM_NAME: &str = "account";

pub fn get_route() -> Vec<Route> {
    routes![add, create, get, remove, register]
}

async fn account_exist(db: &Client, id: &Id) -> Result<bool, AuthError> {
    let mut stream = get_stream(db, id).await?;

    Ok(stream.next().await.is_ok())
}

async fn load_account(db: &Client, id: &Id) -> Result<Account, AuthError> {
    let mut stream = get_stream(db, id).await?;

    let mut account = Account::default();
    let mut exist = false;
    // region iterate-stream
    while let Ok(Some(event)) = stream.next().await {
        exist = true;

        let account_event = event.get_original_event().as_json::<AccountEvent>();

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
        Err(AuthError::NotFound(format!("account `{}` not found", id)))
    }
}

async fn get_stream(db: &Client, id: &Id) -> Result<ReadStream, AuthError> {
    let res = db
        .read_stream(format!("{}-{}", STREAM_NAME, id), &Default::default())
        .await;

    let stream = match res {
        Ok(s) => s,
        Err(err) => {
            return Err(AuthError::Other(format!(
                "Cannot connect to evenstore : {:?}",
                err
            )))
        }
    };
    Ok(stream)
}

async fn add_event(db: &Client, id: &Id, event: AccountEvent) -> Result<(), AuthError> {
    let added = db
        .append_to_stream(
            format!("{}-{}", STREAM_NAME, id),
            &Default::default(),
            event.to_event_data(),
        )
        .await;

    match added {
        Ok(_) => Ok(()),
        Err(err) => Err(AuthError::Other(format!("Cannot add event : {:?}", err))),
    }
}

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
