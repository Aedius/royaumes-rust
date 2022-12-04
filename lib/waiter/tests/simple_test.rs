#![feature(future_join)]

use crate::single::{SingleCommand, SingleNotification, SingleState, GROWTH_STARTED};
use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use waiter::process_wait;

mod single;

#[tokio::test]
async fn single_state_case() {
    let repo = get_repository();
    process_wait::<SingleNotification, SingleState>(repo.clone(), vec![GROWTH_STARTED]).await;

    let key = ModelKey::new("waiter_test".to_string(), Uuid::new_v4().to_string());

    let model = repo.get_model::<SingleState>(&key).await.unwrap();

    assert_eq!(
        model.state(),
        &SingleState {
            nb: 0,
            position: None
        }
    );

    let added = repo
        .add_command::<SingleState>(&key, SingleCommand::Add(15), None)
        .await
        .unwrap();

    assert_eq!(
        added,
        (SingleState {
            nb: 15,
            position: None,
        })
    );

    let growth = repo
        .add_command::<SingleState>(&key, SingleCommand::GrowStart(10, 2), None)
        .await
        .unwrap();

    assert_eq!(
        growth,
        (SingleState {
            nb: 5,
            position: Some(1)
        })
    );

    let secs = Duration::from_secs(4);

    sleep(secs).await;

    let waited = repo.get_model::<SingleState>(&key).await.unwrap();

    assert_eq!(
        waited,
        SingleState {
            nb: 25,
            position: Some(6),
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
