#![allow(unused)]
pub mod dijkstras;
pub mod heatmap;
pub mod parser;

#[cfg(test)]
mod tests;

use core::error;
use std::{
    collections::HashMap,
    mem,
    sync::{Arc, PoisonError, RwLock},
};

use serde::{Deserialize, Serialize};

use gtfs_structures::LocationType;
use thiserror::Error;
use time::{macros::*, Date, Duration, OffsetDateTime, Weekday};

use crate::{coords::Coordinates, gtfs_types::Day};

const SECONDS_IN_DAY: u32 = 86_400;

impl<T> From<PoisonError<T>> for Error {
    fn from(_err: PoisonError<T>) -> Self {
        Self::Poison
    }
}

#[repr(C)]
#[derive(Serialize, Clone, Copy)]
///## Safety
///This must remain as 7 bools, otherwise undefined behaviour happens.
///
/// Must also remain as repr(C)
pub struct ValidDays {
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
}

impl ValidDays {
    fn is_valid(&self, day: Weekday) -> bool {
        match day {
            Weekday::Monday => self.monday,
            Weekday::Tuesday => self.tuesday,
            Weekday::Wednesday => self.wednesday,
            Weekday::Thursday => self.thursday,
            Weekday::Friday => self.friday,
            Weekday::Saturday => self.saturday,
            Weekday::Sunday => self.sunday,
        }
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
    #[error("Couldn't find node with id: {0}")]
    MissingStop(String),
    #[error("The stops location type is not Stop")]
    LocationTypeNotStop,
    #[error("Internal RwLock is poisoned")]
    Poison,
}

#[derive(Serialize)]
pub struct Stop {
    pub id: String,
    #[serde(flatten)]
    pub coordinates: Coordinates,
    #[serde(skip)]
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

#[derive(Serialize)]
struct Edge {
    //Departure and arrival time are not directly from a single stop_time.
    //departure time is from former stop_time and arrival time from latter stop_time
    //used in conjunction with the services date from calendar.
    departure_time: u32,
    #[serde(skip)]
    connected_stop: Arc<RwLock<Stop>>,
    weekdays: ValidDays,
}

impl From<[bool; 7]> for ValidDays {
    ///SAFETY
    /// This is safe, because ValidDays is repr(C) and contains 7 bools.
    fn from(value: [bool; 7]) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl From<ValidDays> for [bool; 7] {
    ///SAFETY
    /// This is safe and sound, because ValidDays is repr(C) and contains 7 bools.
    fn from(value: ValidDays) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl Edge {
    pub fn departure_datetime(&self, current_date_time: OffsetDateTime) -> OffsetDateTime {
        Self::to_datetime(
            &self.departure_time,
            match self.departure_time > SECONDS_IN_DAY
                && self
                    .weekdays
                    .is_valid(current_date_time.date().previous_day().unwrap().weekday())
            {
                true => current_date_time.date().previous_day().unwrap(),
                false => current_date_time.date(),
            },
        )
    }
    pub fn set_day_validity(&mut self, day: Weekday, is_valid: bool) {
        match day {
            Weekday::Monday => self.weekdays.monday = is_valid,
            Weekday::Tuesday => self.weekdays.tuesday = is_valid,
            Weekday::Wednesday => self.weekdays.wednesday = is_valid,
            Weekday::Thursday => self.weekdays.thursday = is_valid,
            Weekday::Friday => self.weekdays.friday = is_valid,
            Weekday::Saturday => self.weekdays.saturday = is_valid,
            Weekday::Sunday => self.weekdays.sunday = is_valid,
        }
    }

    fn get_next_valid_date(&self, date: Date) -> Date {
        let current_day = date.weekday();

        let mut newdays: [bool; 7] = self.weekdays.into();

        newdays.rotate_right(current_day.number_days_from_monday() as usize);
        date + Duration::days(
            newdays
                .iter()
                .position(|x| *x)
                .expect("Edge should have a valid day.") as i64,
        )
    }

    ///Sums a date and a possibly overflowing time.
    ///Time is time of day as seconds, which can be over 24:00
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
#[derive(Serialize)]
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
            weekdays: weekdays.into(),
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
