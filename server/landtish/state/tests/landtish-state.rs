use account_state::event::AccountEvent::ReputationAdded;
use account_state::state::AccountState;

use account_shared::AccountCommand::{AddReputation, RemoveReputation};
use anyhow::Error;
use cucumber::{given, then, when, World};
use state::State;

#[derive(cucumber::World, Debug, Default)]
pub struct AccountWorld {
    model: AccountState,
    err: Option<Error>,
}

#[given(regex = r"^an account with a reputation of (\d+)$")]
fn with_number(world: &mut AccountWorld, rep: usize) {
    world.model.play_event(&ReputationAdded(rep));
}

#[when(regex = r"^i try to add (\d+) reputation$")]
fn add_number(world: &mut AccountWorld, rep: usize) {
    let events = world.model.try_command(&AddReputation(rep));

    match events {
        Ok(list) => {
            for e in list {
                world.model.play_event(&e);
            }
        }
        Err(e) => {
            world.err = Some(e);
        }
    }
}
#[when(regex = r"^i try to remove (\d+) reputation$")]
fn remove_number(world: &mut AccountWorld, rep: usize) {
    let events = world.model.try_command(&RemoveReputation(rep));

    match events {
        Ok(list) => {
            for e in list {
                world.model.play_event(&e);
            }
        }
        Err(e) => {
            world.err = Some(e);
        }
    }
}

#[then(regex = r"^reputation is (\d+)$")]
fn check_number(world: &mut AccountWorld, rep: usize) {
    assert_eq!(rep, world.model.reputation())
}

#[then(regex = r"^i got an error$")]
fn have_error(world: &mut AccountWorld) {
    assert!(world.err.is_some())
}

#[tokio::main]
async fn main() {
    AccountWorld::run("tests/book").await;
}
