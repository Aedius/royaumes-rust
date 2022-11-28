#![feature(future_join)]

use crate::multiple::build::{
    BuildCommand, BuildState, BuildingCreate, ALLOCATION_NEEDED, BUILD_ENDED, BUILD_STARTED,
};
use crate::multiple::gold::{GoldState, PAID};
use crate::multiple::worker::{WorkerState, ALLOCATED, DEALLOCATED};
use crate::multiple::Cost;
use eventstore::Client as EventClient;
use state_repository::{ModelKey, StateRepository};
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use waiter::process_wait;

mod multiple;

#[tokio::test]
async fn multiple_state_case() {
    let repo = get_repository();

    process_wait::<BuildState, BuildState>(repo.clone(), vec!(BUILD_STARTED)).await;

    process_wait::<BuildState, GoldState>(repo.clone(), vec!(ALLOCATION_NEEDED)).await;
    process_wait::<BuildState, WorkerState>(repo.clone(), vec!(ALLOCATION_NEEDED, BUILD_ENDED)).await;

    process_wait::<GoldState, BuildState>(repo.clone(), vec!(PAID)).await;
    process_wait::<WorkerState, BuildState>(repo.clone(), vec!(ALLOCATED,DEALLOCATED)).await;

    let key = ModelKey::new("build_test".to_string(), Uuid::new_v4().to_string());

    let key_bank = ModelKey::new("bank_test".to_string(), Uuid::new_v4().to_string());

    let key_citizen = ModelKey::new("citizen_test".to_string(), Uuid::new_v4().to_string());

    let create = BuildingCreate {
        cost: Cost {
            gold: 322,
            worker: 42,
        },
        bank: key_bank.clone(),
        citizen: key_citizen.clone(),
    };

    let cost = Cost {
        gold: 322,
        worker: 42,
    };

    let build = repo
        .add_command::<BuildState>(&key, BuildCommand::Create(create), None)
        .await
        .unwrap();

    assert_eq!(
        build,
        BuildState {
            cost,
            allocated: Default::default(),
            built: false,
            citizen: Some(key_citizen.clone()),
            bank: Some(key_bank.clone()),
            position: 0,
        }
    );

    sleep(Duration::from_secs(1)).await;

    let state = repo.get_model::<BuildState>(&key).await.unwrap();

    let all_allocated = Cost {
        gold: 322,
        worker: 42,
    };

    assert_eq!(
        state,
        BuildState {
            cost,
            allocated: all_allocated,
            built: false,
            citizen: Some(key_citizen.clone()),
            bank: Some(key_bank.clone()),
            position: 7,
        }
    );

    let worker_state = repo.get_model::<WorkerState>(&key_citizen).await.unwrap();

    assert_eq!(
        worker_state,
        WorkerState {
            nb: 58,
            position: 2,
        }
    );

    let gold_state = repo.get_model::<GoldState>(&key_bank).await.unwrap();

    assert_eq!(
        gold_state,
        GoldState {
            nb: 678,
            position: 2,
        }
    );

    sleep(Duration::from_secs(3)).await;

    let state = repo.get_model::<BuildState>(&key).await.unwrap();
    let worker_freed = Cost {
        gold: 322,
        worker: 0,
    };

    assert_eq!(
        state,
        BuildState {
            cost,
            allocated: worker_freed,
            built: true,
            position: 12,
            citizen: Some(key_citizen.clone()),
            bank: Some(key_bank.clone()),
        }
    );

    let worker_state = repo.get_model::<WorkerState>(&key_citizen).await.unwrap();

    assert_eq!(
        worker_state,
        WorkerState {
            nb: 100,
            position: 5,
        }
    );
    let gold_state = repo.get_model::<GoldState>(&key_bank).await.unwrap();

    assert_eq!(
        gold_state,
        GoldState {
            nb: 678,
            position: 2,
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
