
use crate::cross_state::build::{BuildCommand, BuildState, BuildingCreate};
use crate::cross_state::gold::GoldState;
use eventstore::Client as EventClient;
use state_repository::cross_state::{CrossStateAnswer, CrossStateQuestion};
use state_repository::model_key::ModelKey;
use state_repository::StateRepository;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

mod cross_state;

#[tokio::test]
async fn multiple_state_case() {
    let repo = get_repository();

    let key = ModelKey::new("tower_test".to_string(), Uuid::new_v4().to_string());

    let key_bank = ModelKey::new("bank_test".to_string(), Uuid::new_v4().to_string());

    BuildState::process_query(repo.clone()).await;
    GoldState::process_question(repo.clone()).await;

    sleep(Duration::from_secs(1)).await;

    let create = BuildingCreate {
        cost: 322,
        bank: key_bank.clone(),
    };

    let build = repo
        .add_command::<BuildState>(&key, BuildCommand::Create(create), None)
        .await
        .unwrap();

    assert_eq!(
        build,
        (BuildState {
            cost: 322,
            built: false,
            bank: Some(key_bank.clone()),
        })
    );

    sleep(Duration::from_secs(1)).await;

    let state = repo.get_model::<BuildState>(&key).await.unwrap();

    assert_eq!(
        state.state(),
        &BuildState {
            cost: 322,
            built: true,
            bank: Some(key_bank.clone()),
        }
    );

    let gold_state = repo.get_model::<GoldState>(&key_bank).await.unwrap();

    assert_eq!(gold_state.state(), &GoldState { nb: 678 });
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
