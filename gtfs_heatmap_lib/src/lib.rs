#![feature(async_closure)]

pub mod coords;
pub mod dijkstras;
pub mod gtfs_graph;
pub mod gtfs_types;
pub mod heatmap;

use std::sync::Arc;

use coords::Coordinates;
use futures::executor;
pub use gtfs_structures::Gtfs;
use gtfs_structures::{Stop, StopTime};
use gtfs_types::{seconds_to_hhmmss, Day, StopTrip};

#[derive(Debug)]
pub enum Error {
    ParseError,
    GtfsParseError(gtfs_structures::Error),
}

impl From<gtfs_structures::Error> for Error {
    fn from(value: gtfs_structures::Error) -> Self {
        Self::GtfsParseError(value)
    }
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

pub fn get_next_stops_by_time<'a>(
    time: u32,
    day: &Day,
    stop_id: &String,
    gtfs_data: &'a Gtfs,
) -> Vec<&'a StopTime> {
    gtfs_data
        .trips
        .values()
        .map(|trip| {
            trip.stop_times.iter().filter(|stop_time| {
                if &stop_time.stop.id == stop_id {
                    true
                } else {
                    false
                }
            })
        })
        .flatten()
        .collect()
}

/*
#[test]
fn test_next_stops() {
    let gtfs_data = Gtfs::new("../data/").expect("Missing gtfs data for test");

    let next_stops = get_next_stops_by_time(0, &Day::Monday, &"2111228".to_string(), &gtfs_data);

    dbg!(next_stops);
}
*/
