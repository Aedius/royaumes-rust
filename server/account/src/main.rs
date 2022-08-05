#[macro_use]
extern crate rocket;

use eventstore::Client;
use rocket::response::content;
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};

mod auth;

pub struct EventDb {
    pub db: Client,
}

impl EventDb {
    pub fn new(db: Client) -> EventDb {
        EventDb { db }
    }
}

pub struct MariadDb {
    pub db: Pool<MySql>,
}

impl MariadDb {
    pub fn new(db: Pool<MySql>) -> MariadDb {
        MariadDb { db }
    }
}

#[launch]
fn rocket() -> _ {
    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false"
        .parse()
        .unwrap();
    let event_db = Client::new(settings).unwrap();

    let mariadb_url = "mysql://root:password@localhost:3306/auth";

    let pool = MySqlPool::connect_lazy(mariadb_url).unwrap();

    rocket::build()
        .manage(EventDb::new(event_db))
        .manage(MariadDb::new(pool))
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
