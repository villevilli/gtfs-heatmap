#![feature(async_closure)]

pub mod coords;
pub mod gtfs_graph;
pub mod gtfs_types;

use gtfs_structures::Gtfs;
use std::sync::Arc;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse")]
    ParseError,
}

pub async fn get_stops(gtfs_data: &Gtfs) -> Vec<&Arc<gtfs_structures::Stop>> {
    gtfs_data.stops.values().collect()
}
