use crate::auth::error::AuthError;
use crate::auth::event::{AccountEvent, Created, Quantity};
use crate::auth::{account_exist, add_event, load_account};
use crate::EventDb;
use rocket::State;

#[get("/create/<name>")]
pub async fn create(db_state: &State<EventDb>, name: &str) -> Result<String, AuthError> {
    let db = db_state.db.clone();

    let exist = account_exist(&db, name).await?;
    if exist {
        return Err(AuthError::AlreadyExist(format!(
            "account {} already exist",
            name
        )));
    }

    let payload = AccountEvent::Created(Created {
        name: name.to_string(),
    });

    add_event(&db, name, payload).await?;

    Ok(format!("created {}", name))
}

#[get("/add/<name>/<nb>")]
pub async fn add(db_state: &State<EventDb>, name: &str, nb: usize) -> Result<String, AuthError> {
    let db = db_state.db.clone();

    let account = load_account(&db, name).await?;

    if account.nb.checked_add(nb).is_none() {
        return Err(AuthError::WrongQuantity(format!(
            "cannot add {} to {}",
            nb, account.nb
        )));
    }

    let payload = AccountEvent::Added(Quantity { nb });

    add_event(&db, name, payload).await?;

    Ok(format!("added {} in {}", nb, name))
}

#[get("/remove/<name>/<nb>")]
pub async fn remove(db_state: &State<EventDb>, name: &str, nb: usize) -> Result<String, AuthError> {
    let db = db_state.db.clone();

    let account = load_account(&db, name).await?;

    if nb > account.nb {
        return Err(AuthError::WrongQuantity(format!(
            "cannot remove {} from {}",
            nb, account.nb
        )));
    }

    let payload = AccountEvent::Removed(Quantity { nb });

    add_event(&db, name, payload).await?;

    Ok(format!("added {} in {}", nb, name))
}
