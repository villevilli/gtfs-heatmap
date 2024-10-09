#![allow(unused)]
pub mod parser;

use std::{
    collections::HashMap,
    sync::{Arc, PoisonError, RwLock},
};

use serde::{Deserialize, Serialize};

use gtfs_structures::LocationType;
use thiserror::Error;
use time::{macros::*, Date, Duration, OffsetDateTime};

use crate::{coords::Coordinates, gtfs_types::Day};

const SECONDS_IN_DAY: u32 = 86_400;

impl<T> From<PoisonError<T>> for Error {
    fn from(_err: PoisonError<T>) -> Self {
        Self::Poison
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Stop with id {0} already exsists in graph")]
    DuplicateStop(String),
    #[error("Tried connecting edge to non existing departure node {0}")]
    MissingDepartureStop(String),
    #[error("Tried connecting edge to non existing arrival node {0}")]
    MissingArrivalStop(String),
    #[error("The stops location type is not Stop")]
    LocationTypeNotStop,
    #[error("Internal RwLock is poisoned")]
    Poison,
}

#[derive(Serialize)]
pub struct Stop {
    pub id: String,
    pub coordinates: Coordinates,
    #[serde(skip_serializing)]
    edges: Vec<Arc<Edge>>,
}

///Stops type must be Stop so it can be represented by this
impl TryFrom<gtfs_structures::Stop> for Stop {
    type Error = Error;

    fn try_from(stop: gtfs_structures::Stop) -> Result<Self, Error> {
        if stop.location_type != LocationType::StopPoint {
            return Err(Error::LocationTypeNotStop);
        }

        Ok(Self {
            id: stop.id,
            coordinates: Coordinates {
                latitude: stop
                    .latitude
                    .expect("stop with location type StopPoint always has a latitude"),
                longitude: stop
                    .longitude
                    .expect("stop with location type StopPoint always has a longitude"),
            },
            edges: Vec::new(),
        })
    }
}

struct Edge {
    //Departure and arrival time are not directly from a single stop_time.
    //departure time is from former stop_time and arrival time from latter stop_time
    //used in conjunction with the services date from calendar.
    departure_time: u32,
    connected_stop: Arc<RwLock<Stop>>,
    weekdays: [bool; 7],
}

impl Edge {
    pub fn departure_datetime(&self, current_date: Date) -> OffsetDateTime {
        Self::to_datetime(&self.departure_time, self.get_next_valid_date(current_date))
    }
    pub fn set_day_validity(&mut self, day: Day, is_valid: bool) {
        self.weekdays[day as usize] = is_valid;
    }

    fn get_next_valid_date(&self, date: Date) -> Date {
        let current_day = date.weekday();

        let mut newdays = self.weekdays;
        newdays.rotate_right(current_day.number_days_from_monday() as usize);
        date + Duration::days(
            newdays
                .iter()
                .position(|x| *x)
                .expect("Edge should have a valid day.") as i64,
        )
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

impl GtfsGraph {
    pub fn new() -> Self {
        Self {
            stops: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn insert_stop(&mut self, stop: gtfs_structures::Stop) -> Result<(), Error> {
        if self.stops.contains_key(&stop.id) {
            return Err(Error::DuplicateStop(stop.id));
        }

        self.stops.insert(
            stop.id.clone(),
            Arc::new(RwLock::new(Stop {
                id: stop.id,
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
    ///Connects two stops(nodes)
    ///Valid days is an array of days for which the edge is available. First index is monday.
    pub fn connect_stops(
        &mut self,
        departure_stop_id: &str,
        departure_time: u32,
        arrival_stop_id: &str,
        weekdays: [bool; 7],
    ) -> Result<(), Error> {
        let departure_stop = self
            .stops
            .get_mut(departure_stop_id)
            .ok_or(Error::MissingDepartureStop(departure_stop_id.to_string()))?
            .clone();
        let arrival_stop = self
            .stops
            .get(arrival_stop_id)
            .ok_or(Error::MissingArrivalStop(arrival_stop_id.to_string()))?
            .clone();

        let edge = Arc::new(Edge {
            departure_time,
            connected_stop: arrival_stop,
            weekdays,
        });

        departure_stop.write()?.edges.push(edge.clone());

        self.edges.push(edge);

        Ok(())
    }

    ///Gets stop by its stop_id
    pub fn get_stop(&self, id: &str) -> Option<Arc<RwLock<Stop>>> {
        Some(self.stops.get(id)?.clone())
    }

    pub fn get_stops(&self) -> Vec<Arc<RwLock<Stop>>> {
        self.stops.values().map(|stop| stop.clone()).collect()
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
