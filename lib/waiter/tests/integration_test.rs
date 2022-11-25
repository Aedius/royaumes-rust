#![feature(future_join)]

use crate::state::{WaitCommand, WaitEvent, WaitState};
use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use waiter::process_wait;

mod state;

#[tokio::test]
async fn waiter_case() {
    let repo = get_repository();
    process_wait::<WaitCommand, WaitState>(repo.clone(), WaitEvent::GrowthStarted(0, 0)).await;

    let key = ModelKey::new("waiter_test".to_string(), Uuid::new_v4().to_string());

    let model = repo.get_model::<WaitState>(&key).await.unwrap();

    assert_eq!(model, WaitState { nb: 0, position: 0 });

    let added = repo
        .add_command::<WaitState>(&key, WaitCommand::Add(15))
        .await
        .unwrap();

    assert_eq!(
        added,
        WaitState {
            nb: 15,
            position: 0
        }
    );

    let growth = repo
        .add_command::<WaitState>(&key, WaitCommand::GrowStart(10, 2))
        .await
        .unwrap();

    assert_eq!(growth, WaitState { nb: 5, position: 1 });

    let secs = Duration::from_secs(4);

    sleep(secs).await;

    let waited = repo.get_model::<WaitState>(&key).await.unwrap();

    assert_eq!(
        waited,
        WaitState {
            nb: 25,
            position: 5
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
