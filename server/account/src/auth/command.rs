use crate::auth::error::AuthError;
use crate::auth::event::{AccountEvent, Created, Quantity};
use crate::auth::{account_exist, add_event, load_account, Id};
use crate::{EventDb, MariadDb};
use rocket::State;
use uuid::Uuid;

#[get("/create/<name>")]
pub async fn create(
    event_db: &State<EventDb>,
    maria_db: &State<MariadDb>,
    name: &str,
) -> Result<String, AuthError> {
    let mariadb = maria_db.db.clone();

    let uuid = Uuid::new_v4();

    let id = Id::from(uuid);

    let new_user = sqlx::query!(
        r#"
INSERT INTO `user`
(`uuid`, `email`, `pseudo`, `admin`)
VALUES (?, ?, ?, ?);
        "#,
        id.to_string(),
        id.to_string(),
        name,
        0
    )
    .execute(&mariadb)
    .await;

    if let Err(e) = new_user {
        return Err(AuthError::Other(format!("sql error : {e}")));
    }

    let db = event_db.db.clone();

    let exist = account_exist(&db, &id).await?;
    if exist {
        return Err(AuthError::AlreadyExist(format!(
            "account {} already exist",
            id
        )));
    }

    let payload = AccountEvent::Created(Created { uuid });

    add_event(&db, &id, payload).await?;

    Ok(format!("created {}", id))
}

#[get("/add/<id>/<nb>")]
pub async fn add(event_db: &State<EventDb>, id: Id, nb: usize) -> Result<String, AuthError> {
    let db = event_db.db.clone();

    let account = load_account(&db, &id).await?;

    if account.nb.checked_add(nb).is_none() {
        return Err(AuthError::WrongQuantity(format!(
            "cannot add {} to {}",
            nb, account.nb
        )));
    }

    let payload = AccountEvent::Added(Quantity { nb });

    add_event(&db, &id, payload).await?;

    Ok(format!("added {} in {}", nb, id))
}

#[get("/remove/<id>/<nb>")]
pub async fn remove(event_db: &State<EventDb>, id: Id, nb: usize) -> Result<String, AuthError> {
    let db = event_db.db.clone();

    let account = load_account(&db, &id).await?;

    if nb > account.nb {
        return Err(AuthError::WrongQuantity(format!(
            "cannot remove {} from {}",
            nb, account.nb
        )));
    }

    let payload = AccountEvent::Removed(Quantity { nb });

    add_event(&db, &id, payload).await?;

    Ok(format!("added {} in {}", nb, id))
}
