use crate::auth::jwt_guard::JwtToken;
use crate::{auth};
use account_api::AccountDto;
use account_model::error::AccountError;
use rocket::serde::json::Json;
use rocket::State;
use event_repository::StateRepository;

#[get("/account")]
pub async fn account(
    state_repository: &State<StateRepository>,
    token: JwtToken,
) -> Result<Json<AccountDto>, AccountError> {
    let db = event_db.db.clone();

    let account = auth::load_account(&db, token.uuid).await?;

    Ok(Json(account.dto()))
}

#[get("/header-count")]
pub async fn register(state_repository: &State<StateRepository>) -> String {
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
