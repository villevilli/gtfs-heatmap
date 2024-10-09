use std::{
    collections::HashMap,
    error,
    sync::{Arc, RwLock},
};

use gtfs_structures::Gtfs;

use super::{Error, GtfsGraph, Stop};

impl TryFrom<Gtfs> for GtfsGraph {
    type Error = Error;

    fn try_from(mut gtfs: Gtfs) -> Result<Self, Self::Error> {
        let mut stops: HashMap<String, Arc<RwLock<Stop>>> = HashMap::new();
        for (id, stop) in gtfs.stops.drain() {
            if let Ok(stop) = Arc::unwrap_or_clone(stop).try_into() {
                stops.insert(id, Arc::new(RwLock::new(stop)));
            }
        }

        let mut graph = GtfsGraph {
            stops,
            edges: Vec::new(),
        };

        for (_, trip) in gtfs.trips.drain() {
            let mut iter = trip.stop_times.into_iter().peekable();

            let calendar = gtfs
                .calendar
                .get(&trip.service_id)
                .expect("Trips ServiceID should alway point to a valid instance of calendar");

            let weekdays = [
                calendar.monday,
                calendar.tuesday,
                calendar.wednesday,
                calendar.thursday,
                calendar.friday,
                calendar.saturday,
                calendar.sunday,
            ];

            while let Some(stop) = iter.next() {
                let next_stop = match iter.peek() {
                    Some(stop) => stop,
                    None => break,
                };

                graph.connect_stops(
                    &stop.stop.id,
                    stop.departure_time
                        .expect("stoptime should have departure time"),
                    &next_stop.stop.id,
                    weekdays,
                )?;
            }
        }

        Ok(graph)
    }
}

#[test]
fn gtfs_to_graph() -> Result<(), Box<dyn error::Error>> {
    let mut gtfs = gtfs_structures::Gtfs::from_path("../hsl.zip")?;

    let mut stops: HashMap<String, Arc<RwLock<Stop>>> = HashMap::new();
    for (id, stop) in gtfs.stops.drain() {
        if let Ok(stop) = Arc::unwrap_or_clone(stop).try_into() {
            stops.insert(id, Arc::new(RwLock::new(stop)));
        }
    }
    Ok(())
}

#[test]
fn parse_gtfs_to_graph() -> Result<(), Box<dyn error::Error>> {
    let gtfs = gtfs_structures::Gtfs::from_path("../hsl.zip")?;

    let _gtfs_graph: GtfsGraph = gtfs.try_into()?;

    Ok(())
}
