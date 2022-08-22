use crate::auth::error::AccountError;
use crate::auth::jwt_guard::JwtToken;
use crate::{auth, EventDb};
use rocket::State;

#[get("/account")]
pub async fn account(event_db: &State<EventDb>, token: JwtToken) -> Result<String, AccountError> {
    let db = event_db.db.clone();

    let account = auth::load_account(&db, token.uuid).await?;

    Ok(format!("get : {:?}", account))
}

#[get("/header-count")]
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

    format!("number of header : {:?}", nb)
}
