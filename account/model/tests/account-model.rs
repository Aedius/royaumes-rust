use std::convert::Infallible;
use rocket::futures;
use account_model::Account::Event;
use account_model::event::AccountEvent::Added;
use account_model::event::Quantity;
use account_model::model::AccountModel;

use async_trait::async_trait;
use cucumber::{when, then, given, World, WorldInit};


// `World` is your shared, likely mutable state.
#[derive(Debug, WorldInit)]
pub struct AccountWorld {
    model: AccountModel,
}

#[async_trait(? Send)]
impl World for AccountWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            model: AccountModel::default()
        })
    }
}

#[given("a model with nb 20")]
fn with_number(world: &mut AccountWorld) {
    world.model.nb = 20;
}

#[when("i add the nb 22")]
fn add_number(world: &mut AccountWorld) {
    let event = Event(Added(Quantity { nb: 22 }));

    world.model.play_event(event);
}

#[then("nb is 42")]
fn check_number(world: &mut AccountWorld) {
    assert_eq!(42, world.model.nb)
}

fn main() {
    futures::executor::block_on(AccountWorld::run("tests/book"));
}
