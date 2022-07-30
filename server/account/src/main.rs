
#[macro_use]
extern crate rocket;

use eventstore::Client;

mod auth;

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
    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse().unwrap();
    let event_db = Client::new(settings).unwrap();

    rocket::build()
        .manage(EventDb::new(event_db.clone()))
        .mount("/auth", auth::get_route())
}