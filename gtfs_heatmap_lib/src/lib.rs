#![feature(async_closure)]

pub mod coords;
pub mod dijkstras;
pub mod gtfs_types;
pub mod heatmap;

use std::sync::Arc;

use coords::Coordinates;
use futures::executor;
pub use gtfs_structures::Gtfs;
use gtfs_types::{seconds_to_hhmmss, Day, StopTrip};

#[derive(Debug)]
pub enum Error {
    ParseError,
}

pub async fn get_stops(gtfs_data: &Gtfs) -> Vec<&Arc<gtfs_structures::Stop>> {
    gtfs_data.stops.values().collect()
}

pub async fn get_stop_connection(
    gtfs_data: &Gtfs,
) -> Result<Vec<&Arc<gtfs_structures::Stop>>, Error> {
    todo!();
}

pub async fn get_nearby_stops(
    gtfs_data: &Gtfs,
    coords: &Coordinates,
    search_box_distance: &f64,
) -> Result<Vec<gtfs_types::Stop>, Error> {
    todo!();
}

pub fn get_next_stop_sync(
    trip_id: &String,
    stop_sequence: &u32,
    gtfs_data: &Gtfs,
) -> Result<StopTrip, Error> {
    let next_stop = get_next_stop(trip_id, stop_sequence, gtfs_data);
    executor::block_on(next_stop)
}

pub async fn get_next_stop(
    trip_id: &String,
    stop_sequence: &u32,
    gtfs_data: &Gtfs,
) -> Result<StopTrip, Error> {
    todo!();
}

pub async fn get_next_trips_by_time(
    time: u32,
    day: &Day,
    stop_id: &String,
    gtfs_data: &Gtfs,
) -> Result<Vec<StopTrip>, Error> {
    todo!();
}
