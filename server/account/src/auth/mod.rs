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
use eventstore::Client;
use rocket::Route;

pub fn get_route() -> Vec<Route> {
    routes![add, create, get, remove, register]
}

async fn load_account(db: Client, name: &str) -> Result<Account, AuthError> {
    let res = db
        .read_stream(format!("account-{}", name), &Default::default())
        .await;

    let mut stream = match res {
        Ok(s) => s,
        Err(err) => {
            return Err(AuthError::Other(format!(
                "Cannot connect to evenstore : {:?}",
                err
            )))
        }
    };

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
