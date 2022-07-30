mod event;

use crate::EventDb;
use rocket::{Route, State};
use crate::auth::event::{AccountCreated, AccountEvent};

pub fn get_route() -> Vec<Route> {
    return routes![create, get];
}

#[get("/create/<name>")]
pub async fn create(db_state: &State<EventDb>, name: &str) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Created(AccountCreated{
        name: name.to_string()
    });

    let _ = db
        .append_to_stream(format!("account-{}", name), &Default::default(), payload.to_event_data())
        .await
        .unwrap();


    format!("created {}", name)
}


#[get("/get/<name>")]
pub async fn get(db_state: &State<EventDb>, name: &str) -> String {
    let db = db_state.db.clone();

    let mut  res = db.read_stream(format!("account-{}", name), &Default::default()).await.unwrap();

    // region iterate-stream
    while let Some(event) = res.next().await.unwrap() {
        let test_event = event.get_original_event().as_json::<AccountEvent>().unwrap();

        println!("Event> {:?}", test_event);
    }

    format!("get : {:?}", res)
}