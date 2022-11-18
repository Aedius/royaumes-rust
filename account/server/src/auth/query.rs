use crate::auth::get_key;
use account_shared::AccountDto;
use account_state::error::AccountError;

use crate::AccountIssuer;
use account_state::state::AccountState;
use auth_lib::JwtToken;
use rocket::serde::json::Json;
use rocket::State;
use state_repository::StateRepository;

#[get("/account")]
pub async fn account(
    state_repository: &State<StateRepository>,
    token: JwtToken<AccountIssuer>,
) -> Result<Json<AccountDto>, AccountError> {
    let account = state_repository
        .get_model::<AccountState>(&get_key(Some(token.uuid().to_string())))
        .await
        .unwrap();

    Ok(Json(account.state().dto()))
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
