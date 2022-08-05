use crate::auth::error::AuthError;
use crate::auth::Id;
use crate::{auth, EventDb};
use rocket::State;

#[get("/get/<id>")]
pub async fn get(event_db: &State<EventDb>, id: Id) -> Result<String, AuthError> {
    let db = event_db.db.clone();

    let account = auth::load_account(&db, &id).await?;

    Ok(format!("get : {:?}", account))
}

#[get("/register")]
pub async fn register(event_db: &State<EventDb>) -> String {
    let db = event_db.db.clone();

    let mut res = db
        .read_stream("$et-AccountCreated", &Default::default())
        .await
        .unwrap();

    let mut nb = 0;

    while res.next().await.unwrap().is_some() {
        nb += 1;
    }

    format!("number of register : {:?}", nb)
}