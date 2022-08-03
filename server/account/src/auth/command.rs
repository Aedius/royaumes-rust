use crate::auth::event::{AccountEvent, Created, Quantity};
use crate::EventDb;
use rocket::State;

#[get("/create/<name>")]
pub async fn create(db_state: &State<EventDb>, name: &str) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Created(Created {
        name: name.to_string(),
    });

    let _ = db
        .append_to_stream(
            format!("account-{}", name),
            &Default::default(),
            payload.to_event_data(),
        )
        .await
        .unwrap();

    format!("created {}", name)
}

#[get("/add/<name>/<nb>")]
pub async fn add(db_state: &State<EventDb>, name: &str, nb: usize) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Added(Quantity { nb });

    let _ = db
        .append_to_stream(
            format!("account-{}", name),
            &Default::default(),
            payload.to_event_data(),
        )
        .await
        .unwrap();

    format!("added {} in {}", nb, name)
}

#[get("/remove/<name>/<nb>")]
pub async fn remove(db_state: &State<EventDb>, name: &str, nb: usize) -> String {
    let db = db_state.db.clone();

    let payload = AccountEvent::Removed(Quantity { nb });

    let _ = db
        .append_to_stream(
            format!("account-{}", name),
            &Default::default(),
            payload.to_event_data(),
        )
        .await
        .unwrap();

    format!("added {} in {}", nb, name)
}
