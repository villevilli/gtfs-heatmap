use std::fs::File;
use std::io::Cursor;
use std::sync::RwLock;

use gtfs_heatmap_lib::dijkstras::{StopMapCache, StopNode};
use gtfs_heatmap_lib::gtfs_types::{Day, DayTime, Hour, StopTrip};
use gtfs_heatmap_lib::heatmap::generate_heatmap_tile;
use gtfs_heatmap_lib::{get_stops, gtfs_types, initdb};

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

const DB_LOCATION: &str = "gtfs_db.sqlite";

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Ignite, Request, Response, State};

pub struct CORS;

#[derive(Responder)]
#[response(status = 200, content_type = "image/png")]
struct PngImage(Vec<u8>);

#[derive(Responder)]
#[response(status = 200, content_type = "application/json")]
struct Json(String);

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

#[get("/api/stops")]
fn stops() -> Json {
    let connection = Connection::open(DB_LOCATION).expect("could not open db");
    Json(
        serde_json::to_string(&get_stops(connection).expect("Something went wrong o o"))
            .expect("lol"),
    )
}

#[get("/api/tiles/<stop_id>/<hour>/<day>/<zoom>/<x>/<y>/tile.png")]
fn tiles(
    stop_id: &str,
    hour: u32,
    day: &str,
    zoom: u32,
    x: u32,
    y: u32,
    lookup_table_cache_guard: &State<RwLock<StopMapCache>>,
) -> Option<PngImage> {
    use image::ImageOutputFormat::Png;

    let connection = Connection::open(DB_LOCATION).expect("could not open db");

    let day = day.parse::<Day>().ok()?;

    let time = Hour::try_from(hour).ok()?;

    let lookup_table_cache = lookup_table_cache_guard.read().ok()?;

    match lookup_table_cache.get_if_current(
        DayTime { day, time },
        StopNode {
            stop_id: stop_id.to_string(),
            time_to: 0,
        },
    ) {
        Some(lookup_table) => {
            let tile = generate_heatmap_tile(zoom, x, y, connection, lookup_table);
            let mut writer = Cursor::new(Vec::new());
            tile.write_to(&mut writer, Png).ok()?;
            Some(PngImage(writer.into_inner()))
        }
        None => {
            drop(lookup_table_cache);
            let mut lookup_table_cache = lookup_table_cache_guard.write().ok()?;

            lookup_table_cache.update_lookup_table(
                DayTime { day, time },
                StopNode {
                    stop_id: stop_id.to_string(),
                    time_to: 0,
                },
                &connection,
            );

            let tile = generate_heatmap_tile(zoom, x, y, connection, lookup_table_cache.get());
            let mut writer = Cursor::new(Vec::new());
            tile.write_to(&mut writer, Png).ok()?;
            Some(PngImage(writer.into_inner()))
        }
    }
}

#[launch]
fn rocket() -> _ {
    let stop_map_cache = RwLock::new(StopMapCache::new());

    rocket::build()
        .attach(CORS)
        .manage(stop_map_cache)
        .mount("/", routes![index, stops, tiles])
}
