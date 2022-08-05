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

const STREAM_NAME: &str = "account";

pub fn get_route() -> Vec<Route> {
    routes![add, create, get, remove, register]
}

async fn account_exist(db: &Client, name: &str) -> Result<bool, AuthError> {
    let mut stream = get_stream(db, name).await?;

    Ok(stream.next().await.is_ok())
}

async fn load_account(db: &Client, name: &str) -> Result<Account, AuthError> {
    let mut stream = get_stream(db, name).await?;

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
        Err(AuthError::NotFound(format!("account `{}` not found", name)))
    }
}

async fn get_stream(db: &Client, name: &str) -> Result<ReadStream, AuthError> {
    let res = db
        .read_stream(format!("{}-{}", STREAM_NAME, name), &Default::default())
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

async fn add_event(db: &Client, name: &str, event: AccountEvent) -> Result<(), AuthError> {
    let added = db
        .append_to_stream(
            format!("{}-{}", STREAM_NAME, name),
            &Default::default(),
            event.to_event_data(),
        )
        .await;

    match added {
        Ok(_) => Ok(()),
        Err(err) => Err(AuthError::Other(format!("Cannot add event : {:?}", err))),
    }
}
