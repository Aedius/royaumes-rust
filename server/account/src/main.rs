#[macro_use]
extern crate rocket;

use eventstore::Client;
use rocket::http::Method;
use rocket::response::content;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
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

    let mariadb_url = "mysql://root:password@localhost:3306/account";

    let pool = MySqlPool::connect_lazy(mariadb_url).unwrap();

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3100/"]);

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
        .manage(EventDb::new(event_db))
        .manage(MariadDb::new(pool))
        .mount("/auth", auth::get_route())
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
