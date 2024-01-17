use std::fs::File;
use std::io::Cursor;

use gtfs_heatmap_lib::heatmap::generate_heatmap_tile;
use gtfs_heatmap_lib::{get_stops, initdb};

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

const DB_LOCATION: &str = "gtfs_db.sqlite";

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Ignite, Request, Response};

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

#[get("/api/tiles/<stop_id>/<time>/<day>/<zoom>/<x>/<y>/tile.png")]
fn tiles(stop_id: &str, time: &str, day: &str, zoom: u32, x: u32, y: u32) -> Option<PngImage> {
    let connection = Connection::open(DB_LOCATION).expect("could not open db");

    use image::ImageOutputFormat::Png;

    let tile = generate_heatmap_tile(zoom, x, y, connection);
    let mut writer = Cursor::new(Vec::new());
    tile.write_to(&mut writer, Png).ok()?;
    Some(PngImage(writer.into_inner()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, stops, tiles])
}
