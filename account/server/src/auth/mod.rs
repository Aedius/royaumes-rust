mod command;
mod jwt_guard;
mod query;

use crate::auth::command::handle_anonymous;
use crate::auth::query::{account, register};

use rocket::Route;
use state_repository::model_key::ModelKey;
use uuid::Uuid;

const STREAM_NAME: &str = "account";
const JWT_SECRET: &str = "secret";
const JWT_ISSUER: &str = "royaumes-rs";

pub fn get_route() -> Vec<Route> {
    routes![account, handle_anonymous, register]
}

pub fn get_key(k: Option<String>) -> ModelKey {
    match k {
        None => ModelKey::new(STREAM_NAME.to_string(), Uuid::new_v4().to_string()),
        Some(k) => ModelKey::new(STREAM_NAME.to_string(), k),
    }
}
