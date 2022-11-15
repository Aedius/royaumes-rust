#![feature(future_join)]

use crate::concurrent::{ConcurrentCommand, ConcurrentEvent, ConcurrentState};
use crate::simple::{SimpleCommand, SimpleEvent, SimpleState};
use event_model::{ModelKey, StateRepository};
use eventstore::Client as EventClient;
use std::future::join;
use uuid::Uuid;

mod concurrent;
mod simple;

#[tokio::test]
async fn easy_case() {
    let repo = get_repository();

    let key = ModelKey::new("simple_test".to_string(), Uuid::new_v4().to_string());

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleState>(&key)
        .await
        .unwrap();

    assert_eq!(model, SimpleState { nb: 0, position: 0 });

    let added = repo
        .add_command::<SimpleCommand, SimpleEvent, SimpleState>(&key, SimpleCommand::Add(17))
        .await
        .unwrap();

    assert_eq!(
        added,
        SimpleState {
            nb: 17,
            position: 0
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleState {
            nb: 17,
            position: 1
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleState {
            nb: 17,
            position: 1
        }
    );

    repo.add_command::<SimpleCommand, SimpleEvent, SimpleState>(&key, SimpleCommand::Set(50))
        .await
        .unwrap();

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleState {
            nb: 50,
            position: 4
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleState {
            nb: 50,
            position: 4
        }
    );
}

#[tokio::test]
async fn concurrent_case() {
    let repo = get_repository();

    let key = ModelKey::new("concurrent_test".to_string(), Uuid::new_v4().to_string());

    let model = repo
        .get_model::<ConcurrentCommand, ConcurrentEvent, ConcurrentState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        ConcurrentState {
            names: Vec::new(),
            position: 0
        }
    );

    let add_one = repo.add_command::<ConcurrentCommand, ConcurrentEvent, ConcurrentState>(
        &key,
        ConcurrentCommand::TakeTime(1, "one".to_string()),
    );

    let add_two = repo.add_command::<ConcurrentCommand, ConcurrentEvent, ConcurrentState>(
        &key,
        ConcurrentCommand::TakeTime(2, "two".to_string()),
    );

    let (one, two) = join!(add_one, add_two).await;

    assert!(one.is_ok());
    assert!(two.is_ok());

    let model = repo
        .get_model::<ConcurrentCommand, ConcurrentEvent, ConcurrentState>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        ConcurrentState {
            names: vec!["one".to_string(), "two".to_string()],
            position: 3
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
