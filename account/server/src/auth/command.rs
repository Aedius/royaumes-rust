use crate::auth::{account_exist, add_event, load_account, JWT_ISSUER, JWT_SECRET};
use account_model::error::AccountError;
use account_model::event::{AccountEvent, Created, LoggedIn, Quantity};
use eventstore::EventData;
use jsonwebtokens as jwt;
use jsonwebtokens::{encode, AlgorithmID};
use jwt::Algorithm;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::auth::jwt_guard::JwtToken;
use account_api::{AccountCommand, CreateAccount, Login};
use account_model::model::AccountState;
use account_model::Account;
use event_repository::{ModelKey, StateRepository};

#[post("/", format = "json", data = "<command>")]
pub async fn handle_anonymous(
    state_repository: &State<StateRepository>,
    maria_db: &State<MariadDb>,
    command: Json<AccountCommand>,
    token: Option<JwtToken>,
) -> Result<String, AccountError> {
    match token {
        None => match command.0 {
            AccountCommand::CreateAccount(cmd) => create(state_repository, maria_db, cmd).await,
            AccountCommand::Login(cmd) => login(state_repository, maria_db, cmd).await,
            AccountCommand::AddQuantity(_) => Err(AccountError::Other(
                "cannot add quantity without id".to_string(),
            )),
            AccountCommand::RemoveQuantity(_) => Err(AccountError::Other(
                "cannot remove quantity without id".to_string(),
            )),
            AccountCommand::Join(_) => {
                todo!();
            }
            AccountCommand::Leave(_) => {
                todo!();
            }
        },
        Some(token) => match command.0 {
            AccountCommand::CreateAccount(_) => {
                Err(AccountError::Other("cannot create with id".to_string()))
            }
            AccountCommand::Login(_) => {
                Err(AccountError::Other("cannot login with id".to_string()))
            }
            AccountCommand::AddQuantity(cmd) => {
                let key = ModelKey::new("account".to_string(), token.uuid.clone());
                state_repository
                    .add_command::<AccountCommand, AccountEvent, AccountState>(
                        &key,
                        AccountCommand::AddQuantity(cmd),
                    )
                    .await?;
                Ok("added".to_string())
            }
            AccountCommand::RemoveQuantity(cmd) => {
                let key = ModelKey::new("account".to_string(), token.uuid.clone());
                state_repository
                    .add_command::<AccountCommand, AccountEvent, AccountState>(
                        &key,
                        AccountCommand::RemoveQuantity(cmd),
                    )
                    .await?;
                Ok("removed".to_string())
            }
            AccountCommand::Join(_) => {
                todo!();
            }
            AccountCommand::Leave(_) => {
                todo!();
            }
        },
    }
}

async fn login(
    state_repository: &State<StateRepository>,
    maria_db: &State<MariadDb>,
    cmd: Login,
) -> Result<String, AccountError> {
    let mariadb = maria_db.db.clone();

    let exists = sqlx::query!(
        r#"
SELECT uuid, pseudo FROM `user` WHERE email like ? and password like ? limit 1;
        "#,
        cmd.email,
        cmd.password,
    )
    .fetch_one(&mariadb)
    .await;

    if exists.is_err() {
        return Err(AccountError::Other("sql error".to_string()));
    }

    let db = event_db.db.clone();
    let exists = exists.unwrap();

    let mut events = Vec::new();

    let command = Account::Command(AccountCommand::Login(Login {
        email: "***".to_string(),
        password: "***".to_string(),
        time: 0,
    }))
    .to_event_data(None);

    let correlation_id = command.clone().1;

    events.push(command.0);

    let logged_in = logged_in(correlation_id);

    events.push(logged_in.0);

    add_event(&db, exists.uuid.clone(), events).await?;

    Ok(create_token(exists.uuid))
}

fn logged_in(correlation_id: Uuid) -> (EventData, Uuid) {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time is dead");

    Account::Event(AccountEvent::Logged(LoggedIn {
        time: since_the_epoch.as_secs(),
    }))
    .to_event_data(Some(correlation_id))
}

async fn create(
    state_repository: &State<StateRepository>,
    maria_db: &State<MariadDb>,
    cmd: CreateAccount,
) -> Result<String, AccountError> {
    let mariadb = maria_db.db.clone();
    let uuid = Uuid::new_v4();
    let id = uuid.to_string();

    let exists = sqlx::query!(
        r#"
SELECT email, pseudo FROM `user` WHERE email like ? OR pseudo LIKE ?;
        "#,
        cmd.email,
        cmd.pseudo,
    )
    .fetch_all(&mariadb)
    .await;

    match exists {
        Err(e) => {
            return Err(AccountError::Other(format!("sql error : {e}")));
        }
        Ok(exists) => {
            if !exists.is_empty() {
                for exist in exists {
                    if exist.email == cmd.email {
                        return Err(AccountError::AlreadyExist(format!(
                            "email {} already exists",
                            exist.email
                        )));
                    }
                    if exist.pseudo == cmd.pseudo {
                        return Err(AccountError::AlreadyExist(format!(
                            "pseudo {} already exists",
                            exist.pseudo
                        )));
                    }
                }
            }
        }
    }

    let new_user = sqlx::query!(
        r#"
INSERT INTO `user`
(`uuid`, `email`, `pseudo`, `password`, `admin`)
VALUES (?, ?, ?, ?, ?);
        "#,
        id.to_string(),
        cmd.email,
        cmd.pseudo,
        cmd.password,
        0
    )
    .execute(&mariadb)
    .await;

    if let Err(e) = new_user {
        return Err(AccountError::Other(format!("sql error : {e}")));
    }

    let key = ModelKey::new("account".to_string(), id.clone());

    let command = AccountCommand::CreateAccount(CreateAccount {
        pseudo: cmd.pseudo.clone(),
        email: "***".to_string(),
        password: "***".to_string(),
    });

    let state = state_repository
        .add_command::<AccountCommand, AccountEvent, AccountState>(&key, command)
        .await?;

    Ok(create_token(id))
}

fn create_token(id: String) -> String {
    let alg = Algorithm::new_hmac(AlgorithmID::HS256, JWT_SECRET).unwrap();
    let header = json!({ "alg": alg.name() });
    let claims = json!(JwtToken {
        uuid: id,
        issuer: JWT_ISSUER.to_string()
    });
    encode(&header, &claims, &alg).unwrap()
}
