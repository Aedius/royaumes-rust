mod command;
mod jwt_guard;
mod query;

use crate::auth::command::handle_anonymous;

use account_model::model::AccountModel;
use crate::auth::query::{account, register};

use eventstore::{Client, EventData, ReadStream};
use rocket::Route;
use account_model::Account;
use account_model::error::AccountError;


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

const JWT_SECRET: &str = "secret";
const JWT_ISSUER: &str = "royaumes-rs";
