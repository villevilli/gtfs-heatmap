use rocket::response::Responder;
use rocket::tokio::sync::RwLock;
use std::io::Cursor;

use gtfs_heatmap_lib::dijkstras::{StopMapCache, StopNode};
use gtfs_heatmap_lib::gtfs_types::{Day, DayTime, Hour};
use gtfs_heatmap_lib::heatmap::generate_heatmap_tile;
use gtfs_heatmap_lib::{get_stops, Gtfs};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, State};

#[macro_use]
extern crate rocket;

const DB_CONNECTION: &'static str = "host=localhost user=postgres";

#[derive(Responder)]
#[response(content_type = "text/plain")]
enum Error {
    #[response(status = 500, content_type = "text/plain")]
    PgError(String),
    #[response(status = 500)]
    GtfsErr(()),
    #[response(status = 500)]
    JsonError(()),
}

impl From<gtfs_heatmap_lib::Error> for Error {
    fn from(value: gtfs_heatmap_lib::Error) -> Self {
        match value {
            gtfs_heatmap_lib::Error::ParseError => Self::GtfsErr(()),
        }
    }
}

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
async fn stops(gtfs_data: &State<Gtfs>) -> Result<Json, Error> {
    Ok(Json(
        serde_json::to_string(&get_stops(gtfs_data).await).map_err(|err| Error::JsonError(()))?,
    ))
}

#[get("/api/tiles/<stop_id>/<hour>/<day>/<zoom>/<x>/<y>/tile.png")]
async fn tiles(
    stop_id: &str,
    hour: u32,
    day: &str,
    zoom: u32,
    x: u32,
    y: u32,
    lookup_table_cache_guard: &State<RwLock<StopMapCache>>,
    gtfs_data: &State<Gtfs>,
) -> Option<PngImage> {
    return None;

    use image::ImageFormat::Png;

    let day = day.parse::<Day>().ok()?;

    let time = Hour::try_from(hour).ok()?;

    let lookup_table_cache = lookup_table_cache_guard.read().await;

    match lookup_table_cache.get_if_current(
        DayTime { day, time },
        StopNode {
            stop_id: stop_id.to_string(),
            time_to: 0,
        },
    ) {
        Some(lookup_table) => {
            let tile = generate_heatmap_tile(zoom, x, y, &gtfs_data, lookup_table).await;
            let mut writer = Cursor::new(Vec::new());
            tile.write_to(&mut writer, Png).ok()?;
            Some(PngImage(writer.into_inner()))
        }
        None => {
            drop(lookup_table_cache);
            let mut lookup_table_cache = lookup_table_cache_guard.write().await;

            lookup_table_cache.update_lookup_table(
                DayTime { day, time },
                StopNode {
                    stop_id: stop_id.to_string(),
                    time_to: 0,
                },
                &gtfs_data,
            );

            let tile =
                generate_heatmap_tile(zoom, x, y, &gtfs_data, lookup_table_cache.get()).await;
            let mut writer = Cursor::new(Vec::new());
            tile.write_to(&mut writer, Png).ok()?;
            Some(PngImage(writer.into_inner()))
        }
    }
}

#[launch]
fn rocket() -> _ {
    let stop_map_cache = RwLock::new(StopMapCache::new());
    let gtfs_data = Gtfs::from_path("data/").expect("GTFS data should exsist in \"data/\" folder");

    rocket::build()
        .attach(CORS)
        .manage(stop_map_cache)
        .manage(gtfs_data)
        .mount("/", routes![index, stops, tiles])
}
