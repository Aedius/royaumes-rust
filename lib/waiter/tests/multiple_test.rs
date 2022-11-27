#![feature(future_join)]

use crate::multiple::build::{BuildCommand, BuildState};
use crate::multiple::Cost;
use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

mod multiple;

#[tokio::test]
async fn multiple_state_case() {
    let repo = get_repository();

    let key = ModelKey::new("build_test".to_string(), Uuid::new_v4().to_string());

    let cost = Cost {
        gold: 322,
        worker: 42,
    };

    let build = repo
        .add_command::<BuildState>(&key, BuildCommand::Create(cost), None)
        .await
        .unwrap();

    assert_eq!(
        build,
        BuildState {
            cost,
            allocated: Default::default(),
            built: false,
            position: 0,
        }
    );

    let secs = Duration::from_secs(4);

    sleep(secs).await;

    let cost_gold = Cost {
        gold: 322,
        worker: 0,
    };

    assert_eq!(
        build,
        BuildState {
            cost,
            allocated: cost_gold,
            built: true,
            position: 0,
        }
    );
}

fn get_repository() -> StateRepository {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .unwrap();
    let event_db = EventClient::new(settings).unwrap();

    let cache_db = redis::Client::open("redis://localhost:6379/").unwrap();

    let repo = StateRepository::new(event_db, cache_db);

    repo
}
