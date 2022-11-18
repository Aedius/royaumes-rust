#[macro_use]
extern crate rocket;

use auth_lib::Issuer;
use dotenvy::dotenv;
use eventstore::Client;
use global_config::Components::{Private, Public};
use global_config::Config;
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use state_repository::StateRepository;

mod auth;

pub struct MariadDb {
    pub db: Pool<MySql>,
}

impl MariadDb {
    pub fn new(db: Pool<MySql>) -> MariadDb {
        MariadDb { db }
    }
}

pub struct AccountIssuer {}

impl Issuer for AccountIssuer {
    fn name() -> String {
        dotenvy::var("ISSUER_ACCOUNT_NAME").unwrap()
    }

    fn secret() -> String {
        dotenvy::var("ISSUER_ACCOUNT_SECRET").unwrap()
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    AccountIssuer::name();
    AccountIssuer::secret();

    let config = Config::load();

    // Creates a public settings for a single node configuration.
    let settings = config.event_store().parse().unwrap();
    let event_db = Client::new(settings).unwrap();

    let cache_db = redis::Client::open(config.redis()).unwrap();

    let state_repository = StateRepository::new(event_db, cache_db);

    let mariadb_url = format!("{}/account", config.mysql());
    let pool = MySqlPool::connect_lazy(&mariadb_url).unwrap();

    let allowed_origins = AllowedOrigins::some_exact(&[
        config.get_uri(Public).unwrap(),
        config.get_uri(Private).unwrap(),
    ]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .manage(state_repository)
        .manage(MariadDb::new(pool))
        .mount("/api", auth::get_route())
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .register("/", catchers![general_not_found])
}

#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <p>Hmm... This is not the droïd you are looking for</p>
    "#,
    )
}
