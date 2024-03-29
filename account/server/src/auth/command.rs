use crate::auth::get_key;
use account_state::error::AccountError;

use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;

use crate::{AccountIssuer, MariadDb};
use account_shared::{AccountCommand, CreateAccount, Login};
use account_state::state::AccountState;
use auth_lib::JwtToken;
use state_repository::model_key::ModelKey;
use state_repository::StateRepository;

#[post("/", format = "json", data = "<command>")]
pub async fn handle_anonymous(
    state_repository: &State<StateRepository>,
    maria_db: &State<MariadDb>,
    command: Json<AccountCommand>,
    token: Option<JwtToken<AccountIssuer>>,
) -> Result<String, AccountError> {
    match token {
        None => match command.0 {
            AccountCommand::CreateAccount(cmd) => create(state_repository, maria_db, cmd).await,
            AccountCommand::Login(cmd) => login(state_repository, maria_db, cmd).await,
            AccountCommand::AddReputation(_) => Err(AccountError::Other(
                "cannot add quantity without id".to_string(),
            )),
            AccountCommand::RemoveReputation(_) => Err(AccountError::Other(
                "cannot remove quantity without id".to_string(),
            )),
        },
        Some(token) => match command.0 {
            AccountCommand::CreateAccount(_) => {
                Err(AccountError::Other("cannot create with id".to_string()))
            }
            AccountCommand::Login(_) => {
                Err(AccountError::Other("cannot login with id".to_string()))
            }
            AccountCommand::AddReputation(cmd) => {
                let key = ModelKey::new("account".to_string(), token.uuid().to_string());
                state_repository
                    .add_command::<AccountState>(&key, AccountCommand::AddReputation(cmd), None)
                    .await?;
                Ok("added".to_string())
            }
            AccountCommand::RemoveReputation(cmd) => {
                let key = ModelKey::new("account".to_string(), token.uuid().to_string());
                state_repository
                    .add_command::<AccountState>(&key, AccountCommand::RemoveReputation(cmd), None)
                    .await?;
                Ok("removed".to_string())
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
    let exists = exists.unwrap();

    let command = AccountCommand::Login(Login {
        email: "***".to_string(),
        password: "***".to_string(),
    });

    state_repository
        .add_command::<AccountState>(&get_key(Some(exists.uuid.clone())), command, None)
        .await?;

    Ok(JwtToken::<AccountIssuer>::create(exists.uuid))
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

    state_repository
        .add_command::<AccountState>(&key, command, None)
        .await?;

    Ok(JwtToken::<AccountIssuer>::create(id))
}
