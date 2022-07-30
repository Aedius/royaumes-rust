mod event;
mod model;

use crate::EventDb;
use rocket::{Route, State};
use crate::auth::event::{Created, AccountEvent, Quantity};
use crate::auth::model::Account;

pub fn get_route() -> Vec<Route> {
    return routes![add, create, get];
}

#[get("/create/<name>")]
pub async fn create(db_state: &State<EventDb>, name: &str) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Created(Created {
        name: name.to_string()
    });

    let _ = db
        .append_to_stream(format!("account-{}", name), &Default::default(), payload.to_event_data())
        .await
        .unwrap();


    format!("created {}", name)
}

#[get("/add/<name>/<nb>")]
pub async fn add(db_state: &State<EventDb>, name: &str, nb: usize) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Added(Quantity {
        nb
    });

    let _ = db
        .append_to_stream(format!("account-{}", name), &Default::default(), payload.to_event_data())
        .await
        .unwrap();


    format!("added {} in {}",nb, name)
}


#[get("/get/<name>")]
pub async fn get(db_state: &State<EventDb>, name: &str) -> String {
    let db = db_state.db.clone();

    let mut  res = db.read_stream(format!("account-{}", name), &Default::default()).await.unwrap();

    let mut account = Account::default();

    // region iterate-stream
    while let Some(event) = res.next().await.unwrap() {
        let test_event = event.get_original_event().as_json::<AccountEvent>().unwrap();

        println!("event : {:?}", event);
        println!("test_event : {:?}", test_event);
        account.play_event(test_event).unwrap();
    }

    format!("get : {:?}", account)
}