use bank_state::event::bankEvent::Joined;
use bank_state::state::bankState;

use bank_shared::bankCommand::Join;
use anyhow::Error;
use cucumber::{given, then, when, World};
use state::State;


#[derive(cucumber::World, Debug, Default)]
pub struct BankWorld {
    model: bankState,
    err: Option<Error>,
}

#[tokio::main]
async fn main() {
    bankWorld::run("tests/book").await;
}