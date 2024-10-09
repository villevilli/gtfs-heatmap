use gtfs_heatmap_lib::gtfs_graph::GtfsGraph;
use rocket::response::Responder;
use rocket::time::OffsetDateTime;

use gtfs_heatmap_lib::Gtfs;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, State};

#[macro_use]
extern crate rocket;

#[derive(Responder)]
#[response(content_type = "text/plain")]
enum Error {
    #[response(status = 500, content_type = "text/plain")]
    GtfsErr(String),
    #[response(status = 500, content_type = "text/plain")]
    JsonError(String),
}

impl From<gtfs_heatmap_lib::Error> for Error {
    fn from(value: gtfs_heatmap_lib::Error) -> Self {
        Self::GtfsErr(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value.to_string())
    }
}

impl From<gtfs_heatmap_lib::gtfs_graph::Error> for Error {
    fn from(value: gtfs_heatmap_lib::gtfs_graph::Error) -> Self {
        Self::GtfsErr(value.to_string())
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
async fn stops(gtfs_data: &State<GtfsGraph>) -> Result<Json, Error> {
    Ok(Json(serde_json::to_string(&gtfs_data.get_stops())?))
}

#[get("/api/stops/<stop_id>/dijkstras/<timestamp>")]
async fn dijkstras(
    stop_id: &str,
    timestamp: i64,
    gtfs_data: &State<GtfsGraph>,
) -> Result<Json, Error> {
    let times = gtfs_data.dijkstras(
        stop_id,
        OffsetDateTime::from_unix_timestamp(timestamp).expect("lol"),
    )?;

    Ok(Json(serde_json::to_string(&times)?))
}

#[allow(unused_variables)]
#[get("/api/tiles/<stop_id>/<hour>/<day>/<zoom>/<x>/<y>/tile.png")]
async fn tiles(
    stop_id: &str,
    hour: u32,
    day: &str,
    zoom: u32,
    x: u32,
    y: u32,
    gtfs_data: &State<GtfsGraph>,
) -> Option<PngImage> {
    return None;
    /*
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
    */
}

/*
#[get("/api/graph")]
fn get_graph(gtfs_data: &State<GtfsGraph>) -> Result<Json, Error> {
    Ok(Json(gtfs_data.serialize(Json)?))
}
*/
#[launch]
fn rocket() -> _ {
    let gtfs_data: GtfsGraph = Gtfs::from_path("data/")
        .expect("GTFS data should exsist in \"data/\" folder")
        .try_into()
        .expect("Should just work??");

    rocket::build()
        .attach(CORS)
        .manage(gtfs_data)
        .mount("/", routes![index, stops, tiles, dijkstras])
}
