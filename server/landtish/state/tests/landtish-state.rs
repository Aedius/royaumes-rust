use landtish_state::event::LandtishEvent::Joined;
use landtish_state::state::LandtishState;

use landtish_shared::LandtishCommand::Join;
use anyhow::Error;
use cucumber::{given, then, when, World};
use state::State;


#[derive(cucumber::World, Debug, Default)]
pub struct LandtishWorld {
    model: LandtishState,
    err: Option<Error>,
}

#[tokio::main]
async fn main() {
    LandtishWorld::run("tests/book").await;
}