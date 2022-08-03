use crate::auth::error::AuthError;
use crate::{auth, EventDb};
use rocket::State;

#[get("/get/<name>")]
pub async fn get(db_state: &State<EventDb>, name: &str) -> Result<String, AuthError> {
    let db = db_state.db.clone();

    let account = auth::load_account(db, name).await?;

    Ok(format!("get : {:?}", account))
}

#[get("/register")]
pub async fn register(db_state: &State<EventDb>) -> String {
    let db = db_state.db.clone();

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
