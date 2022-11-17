use account_state::event::AccountEvent::{AccountAdded, Joined};
use account_state::event::{Quantity, ServerAccount};
use account_state::state::AccountState;

use account_shared::{AccountCommand, ServerAccount as SA};
use cucumber::{given, then, when, World};
use state::State;

#[derive(cucumber::World, Debug, Default)]
pub struct AccountWorld {
    model: AccountState,
}

#[given(regex = r"^a state with nb (\d+)$")]
fn with_number(world: &mut AccountWorld, account: usize) {
    world
        .model
        .play_event(&AccountAdded(Quantity { nb: account }));
}

#[when(regex = r"^i add the nb (\d+)$")]
fn add_number(world: &mut AccountWorld, account: usize) {
    world
        .model
        .play_event(&AccountAdded(Quantity { nb: account }));
}

#[when(regex = r"^i have joined the server (.*) with account (.*)$")]
fn join_server(world: &mut AccountWorld, server_id: String, account_id: String) {
    world.model.play_event(&Joined(ServerAccount {
        server_id,
        account_id,
    }));
}

#[then(regex = r"^nb is (\d+)$")]
fn check_number(world: &mut AccountWorld, account: usize) {
    assert_eq!(account, world.model.nb_account_allowed())
}

#[then(regex = r"^i have joined (\d+) server$")]
fn joined(world: &mut AccountWorld, account: usize) {
    assert_eq!(account, world.model.nb_accounts())
}

#[then(regex = r"^i can leave the server (.*) with account (.*)$")]
fn can_leave(world: &mut AccountWorld, server_id: String, account_id: String) {
    let res = match world.model.try_command(&AccountCommand::Leave(SA {
        server_id,
        account_id,
    })) {
        Ok(_) => 1,
        Err(_e) => 2,
    };

    assert_eq!(res, 1)
}

#[then(regex = r"^i cant join the server (.*) with account (.*)$")]
fn cant_join(world: &mut AccountWorld, server_id: String, account_id: String) {
    let res = match world.model.try_command(&AccountCommand::Join(SA {
        server_id,
        account_id,
    })) {
        Ok(_) => 1,
        Err(_e) => 2,
    };

    assert_eq!(res, 2)
}

#[tokio::main]
async fn main() {
    AccountWorld::run("tests/book").await;
}
