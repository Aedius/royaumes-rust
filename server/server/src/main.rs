#[macro_use]
extern crate rocket;

use eventstore::Client;
use global_config::Components::{Public, Server};
use global_config::Config;
use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket::response::content;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub struct EventDb {
    pub db: Client,
}

impl EventDb {
    pub fn new(db: Client) -> EventDb {
        EventDb { db }
    }
}

#[launch]
fn rocket() -> _ {
    let config = Config::load();
    // Creates a public settings for a single node configuration.
    let settings = config.event_store().parse().unwrap();
    let event_db = Client::new(settings).unwrap();

    let allowed_origins = AllowedOrigins::some_exact(&[config.get_uri(Public).unwrap()]);

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

    let figment = rocket::Config::figment().merge(("port", config.get_port(Server)));

    rocket::custom(figment)
        .manage(EventDb::new(event_db))
        .mount("/", FileServer::from(relative!("web")))
        .attach(cors)
        .register("/", catchers![general_not_found])
}

#[catch(404)]
fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(
        r#"
        <p>404 error not found</p>
    "#,
    )
}
