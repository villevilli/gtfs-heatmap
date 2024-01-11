use gtfs_heatmap_lib::{get_stops, initdb};

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

const DB_LOCATION: &str = "gtfs_db.sqlite";

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Ignite, Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/stops", format = "json")]
fn stops() -> String {
    let connection = Connection::open(DB_LOCATION).expect("could not open db");
    serde_json::to_string(&get_stops(connection).expect("Something went wrong o o")).expect("lol")
}

#[get("/api/tiles/<zoom>/<x>/<y>")]
fn tiles(zoom: u32, x: u32, y: u32) {}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, stops])
}
