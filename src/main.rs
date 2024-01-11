use gtfs_heatmap_lib::get_stops;
use rocket::response::content::RawJson;
use rusqlite::Connection;

#[macro_use]
extern crate rocket;

const DB_LOCATION: &str = "gtfs_db.sqlite";

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/stops", format = "json")]
fn stops() -> String {
    let connection = Connection::open(DB_LOCATION).expect("could not open db");
    serde_json::to_string(&get_stops(connection).expect("Something went wrong o o")).expect("lol")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![stops])
}
