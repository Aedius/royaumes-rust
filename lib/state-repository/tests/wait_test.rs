#![feature(future_join)]

use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use tokio::time::{Duration, sleep};
use uuid::Uuid;
use state_repository::waiter::DelayedState;
use crate::wait::{WaitCommand, WaitState};

mod wait;

#[tokio::test]
async fn wait_case() {
    let repo = get_repository();
    WaitState::process_delayed(repo.clone()).await;

    let key = ModelKey::new("waiter_test".to_string(), Uuid::new_v4().to_string());

    let model = repo.get_model::<WaitState>(&key).await.unwrap();

    assert_eq!(
        model.state(),
        &WaitState {
            nb: 0,
        }
    );

    let added = repo
        .add_command::<WaitState>(&key, WaitCommand::Add(15), None)
        .await
        .unwrap();

    assert_eq!(
        added,
        (WaitState {
            nb: 15,
        })
    );

    let growth = repo
        .add_command::<WaitState>(&key, WaitCommand::Growth(10), None)
        .await
        .unwrap();

    assert_eq!(
        growth,
        (WaitState {
            nb: 5,
        })
    );

    let secs = Duration::from_secs(3);

    sleep(secs).await;

    let waited = repo.get_model::<WaitState>(&key).await.unwrap();

    assert_eq!(
        waited.state(),
        &WaitState {
            nb: 25,
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
