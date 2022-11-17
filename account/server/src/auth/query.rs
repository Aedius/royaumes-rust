use crate::auth::get_key;
use crate::auth::jwt_guard::JwtToken;
use account_api::AccountDto;
use account_model::error::AccountError;

use account_model::model::AccountState;
use event_repository::StateRepository;
use rocket::serde::json::Json;
use rocket::State;

#[get("/account")]
pub async fn account(
    state_repository: &State<StateRepository>,
    token: JwtToken,
) -> Result<Json<AccountDto>, AccountError> {
    let account = state_repository
        .get_model::<AccountState>(&get_key(Some(token.uuid)))
        .await
        .unwrap();

    Ok(Json(account.dto()))
}

#[get("/header-count")]
pub async fn register(state_repository: &State<StateRepository>) -> String {
    let db = state_repository.event_db().clone();

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
