#[macro_use]
extern crate rocket;

use eventstore::Client;
use rocket::response::content;

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
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .parse()
        .unwrap();
    let event_db = Client::new(settings).unwrap();

    rocket::build()
        .manage(EventDb::new(event_db))
        .mount("/auth", auth::get_route())
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
