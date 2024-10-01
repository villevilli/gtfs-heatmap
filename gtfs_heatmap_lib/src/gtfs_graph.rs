use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, PoisonError, RwLock},
};

use gtfs_structures::RawGtfs;
use time::{
    macros::{date, datetime, time},
    Date, Duration, OffsetDateTime, Time,
};

use crate::coords::Coordinates;

const SECONDS_IN_DAY: u32 = 86_400;

pub enum Error {
    DuplicateStopError,
    MissingDepartureStop,
    MissingArrivalStop,
    StopPoisonError,
}

///Poison Errors contents are deleted here, couldn't be fucked to add type parameter
impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Self::StopPoisonError
    }
}

struct Stop {
    id: String,
    coordinates: Coordinates,
    edges: Vec<Arc<Edge>>,
}

struct Edge {
    //Departure and arrival time are not directly from a single stop_time.
    //departure time is from former stop_time and arrival time from latter stop_time
    //used in conjunction with the services date from calendar.
    departure_time: u32,
    arrival_time: u32,
    connected_stop: Arc<RwLock<Stop>>,
}

impl Edge {
    pub fn departure_datetime(&self, current_date: Date) -> OffsetDateTime {
        Self::to_datetime(&self.arrival_time, current_date)
    }
    pub fn arrival_datetime(&self, current_date: Date) -> OffsetDateTime {
        Self::to_datetime(&self.departure_time, current_date)
    }

    fn to_datetime(time: &u32, date: Date) -> OffsetDateTime {
        match time / SECONDS_IN_DAY {
            0 => OffsetDateTime::new_utc(date, time!(00:00:00)) + Duration::seconds(*time as i64),
            day_overflow => {
                OffsetDateTime::new_utc(date + Duration::days(day_overflow as i64), time!(00:00:00))
                    + Duration::seconds((time - (SECONDS_IN_DAY * day_overflow)) as i64)
            }
        }
    }
}

/// Stops are stored as HashMap with stop_id as Key.
pub struct GtfsGraph {
    stops: HashMap<String, Arc<RwLock<Stop>>>,
    edges: Vec<Arc<Edge>>,
}

impl TryFrom<RawGtfs> for GtfsGraph {
    type Error = gtfs_structures::Error;

    fn try_from(value: RawGtfs) -> Result<Self, Self::Error> {
        let graph = Self::new();

        Ok(graph)
    }
}

impl GtfsGraph {
    pub fn new() -> Self {
        Self {
            stops: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn insert_stop(&mut self, stop: &gtfs_structures::Stop) -> Result<(), Error> {
        if self.stops.contains_key(&stop.id) {
            return Err(Error::DuplicateStopError);
        }

        self.stops.insert(
            stop.id.clone(),
            Arc::new(RwLock::new(Stop {
                id: stop.id.clone(),
                coordinates: Coordinates {
                    latitude: stop
                        .latitude
                        .expect("GTFS DATA CONTAINS STOP WITHOUT LATITUDE, should fix lol"),
                    longitude: stop
                        .longitude
                        .expect("GTFS DATA CONTAINS STOP WITHOUT LONGITUDE, should fix lol"),
                },
                edges: Vec::new(),
            })),
        );

        Ok(())
    }

    pub fn connect_stops(
        &mut self,
        departure_stop_id: String,
        departure_time: u32,
        arrival_stop_id: String,
        arrival_time: u32,
    ) -> Result<(), Error> {
        let departure_stop = self
            .stops
            .get_mut(&departure_stop_id)
            .ok_or(Error::MissingDepartureStop)?
            .clone();
        let arrival_stop = self
            .stops
            .get(&arrival_stop_id)
            .ok_or(Error::MissingArrivalStop)?
            .clone();

        let edge = Arc::new(Edge {
            departure_time,
            arrival_time,
            connected_stop: arrival_stop,
        });

        departure_stop.write()?.edges.push(edge.clone());

        self.edges.push(edge);

        Ok(())
    }
}

#[test]
fn to_datetime_midnight() {
    let date = date!(2003 - 5 - 16);
    let time = SECONDS_IN_DAY;

    let datetime = Edge::to_datetime(&time, date);

    assert_eq!(datetime, datetime!(2003 - 5 - 17 0:00 UTC));
}

#[test]
fn to_datetime_past_midnight() {
    let date = date!(2003 - 5 - 16);
    let time = SECONDS_IN_DAY + 120;

    let datetime = Edge::to_datetime(&time, date);

    assert_eq!(datetime, datetime!(2003 - 5 - 17 0:02 UTC))
}
