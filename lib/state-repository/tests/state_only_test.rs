#![feature(future_join)]

use crate::concurrent::{ConcurrentCommand, ConcurrentState};
use crate::simple::{SimpleCommand, SimpleState};

use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use std::future::join;
use uuid::Uuid;

mod concurrent;
mod simple;

#[tokio::test]
async fn easy_case() {
    let repo = get_repository();

    let key = ModelKey::new("simple_test".to_string(), Uuid::new_v4().to_string());

    let model = repo.get_model::<SimpleState>(&key).await.unwrap();

    assert_eq!(model.state(), &SimpleState { nb: 0 });

    let added = repo
        .add_command::<SimpleState>(&key, SimpleCommand::Add(17), None)
        .await
        .unwrap();

    assert_eq!(added, (SimpleState { nb: 17 }));

    let model = repo.get_model::<SimpleState>(&key).await.unwrap();

    assert_eq!(model.state(), &SimpleState { nb: 17 });

    let model = repo.get_model::<SimpleState>(&key).await.unwrap();

    assert_eq!(model.state(), &SimpleState { nb: 17 });

    repo.add_command::<SimpleState>(&key, SimpleCommand::Set(50), None)
        .await
        .unwrap();

    let model = repo.get_model::<SimpleState>(&key).await.unwrap();

    assert_eq!(model.state(), &SimpleState { nb: 50 });

    let model = repo.get_model::<SimpleState>(&key).await.unwrap();

    assert_eq!(model.state(), &SimpleState { nb: 50 });
}

#[tokio::test]
async fn concurrent_case() {
    let repo = get_repository();

    let key = ModelKey::new("concurrent_test".to_string(), Uuid::new_v4().to_string());

    let model = repo.get_model::<ConcurrentState>(&key).await.unwrap();

    assert_eq!(
        model.state(),
        &ConcurrentState {
            names: Vec::new(),
            position: None
        }
    );

    let add_one = repo.add_command::<ConcurrentState>(
        &key,
        ConcurrentCommand::TakeTime(1, "one".to_string()),
        None,
    );

    let add_two = repo.add_command::<ConcurrentState>(
        &key,
        ConcurrentCommand::TakeTime(2, "two".to_string()),
        None,
    );

    let (one, two) = join!(add_one, add_two).await;

    assert!(one.is_ok());
    assert!(two.is_ok());

    let model = repo.get_model::<ConcurrentState>(&key).await.unwrap();

    assert_eq!(
        model.state(),
        &ConcurrentState {
            names: vec!["one".to_string(), "two".to_string()],
            position: Some(3)
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
