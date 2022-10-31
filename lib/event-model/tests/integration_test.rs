use crate::definition::{SimpleCommand, SimpleEvent, SimpleModel};
use event_model::{ModelKey, ModelRepository};
use eventstore::Client as EventClient;
use uuid::Uuid;

mod definition;

#[tokio::test]
async fn easy_case() {
    let repo = get_repository();

    let key = ModelKey::new("simple_test".to_string(), Uuid::new_v4().to_string());

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleModel>(&key)
        .await
        .unwrap();

    assert_eq!(model, SimpleModel { nb: 0, position: 0 });

    let added = repo
        .add_command::<SimpleCommand, SimpleEvent, SimpleModel>(&key, SimpleCommand::Add(17))
        .await
        .unwrap();

    assert_eq!(
        added,
        SimpleModel {
            nb: 17,
            position: 0
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleModel>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleModel {
            nb: 17,
            position: 1
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleModel>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleModel {
            nb: 17,
            position: 1
        }
    );

    repo.add_command::<SimpleCommand, SimpleEvent, SimpleModel>(&key, SimpleCommand::Set(50))
        .await
        .unwrap();

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleModel>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleModel {
            nb: 50,
            position: 4
        }
    );

    let model = repo
        .get_model::<SimpleCommand, SimpleEvent, SimpleModel>(&key)
        .await
        .unwrap();

    assert_eq!(
        model,
        SimpleModel {
            nb: 50,
            position: 4
        }
    );
}

fn get_repository() -> ModelRepository {
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .to_string()
        .parse()
        .unwrap();
    let event_db = EventClient::new(settings).unwrap();

    let cache_db = redis::Client::open("redis://localhost:6379/").unwrap();

    let repo = ModelRepository::new(event_db, cache_db);

    repo
}
