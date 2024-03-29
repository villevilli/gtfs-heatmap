use std::fmt::{self, format};
use std::str::FromStr;

use deadpool_postgres::tokio_postgres;
use deadpool_postgres::tokio_postgres::Row;
use serde::{Deserialize, Serialize};

use crate::coords::Coordinates;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StopTrip {
    pub stop_id: String,
    pub trip_id: String,
    pub stop_sequence: u32,
    pub arrival_time: u32,
    pub departure_time: u32,
}

impl TryFrom<&Row> for StopTrip {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<StopTrip, tokio_postgres::Error> {
        let arrival_time_string: String = row.try_get::<usize, String>(3)?;
        let departure_time_string: String = row.try_get::<usize, String>(3)?;

        let mut stop_trip = StopTrip {
            stop_id: row.try_get::<usize, String>(0)?,
            trip_id: row.try_get::<usize, String>(1)?,
            stop_sequence: row.try_get::<usize, u32>(2)?,
            arrival_time: parse_time(&arrival_time_string),
            departure_time: parse_time(&departure_time_string),
        };

        Ok(stop_trip)
    }
}

pub fn parse_time(string: &str) -> u32 {
    let mut splits = string.split(":");

    let mut num = 0;

    num += splits.next().unwrap().parse::<u32>().unwrap() * 3600;
    num += splits.next().unwrap().parse::<u32>().unwrap() * 60;
    num += splits.next().unwrap().parse::<u32>().unwrap() * 1;

    num
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    pub stop_id: String,
    pub stop_code: String,
    pub stop_name: String,
    pub stop_desc: String,
    pub coordinates: Coordinates,
    pub zone_id: String,
    pub stop_url: String,
    pub location_type: u32,
    pub parent_station: String,
    pub wheelchair_boarding: u32,
}
impl TryFrom<&Row> for Stop {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Stop, tokio_postgres::Error> {
        Ok(Stop {
            stop_id: row.try_get::<usize, String>(0)?,
            stop_code: row.try_get::<usize, String>(1)?,
            stop_name: row.try_get::<usize, String>(2)?,
            stop_desc: row.try_get::<usize, String>(3)?,
            coordinates: Coordinates {
                latitude: row.try_get::<usize, f64>(4)?,
                longitude: row.try_get::<usize, f64>(5)?,
            },
            zone_id: row.try_get::<usize, String>(6)?,
            stop_url: row.try_get::<usize, String>(7)?,
            location_type: row.try_get::<usize, u32>(8)?,
            parent_station: row.try_get::<usize, String>(9)?,
            wheelchair_boarding: row.try_get::<usize, u32>(10)?,
        })
    }
}

#[derive(Hash, PartialEq, Eq, Default, Clone, Copy, Debug)]
pub enum Day {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl FromStr for Day {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mon" => Ok(Self::Monday),
            "Tue" => Ok(Self::Tuesday),
            "Wen" => Ok(Self::Wednesday),
            "Thu" => Ok(Self::Thursday),
            "Fri" => Ok(Self::Friday),
            "Sat" => Ok(Self::Saturday),
            "Sun" => Ok(Self::Sunday),
            _ => Err(ParseError),
        }
    }
}

//display is in finnish because we are dealing with hsl's gtfs data where days are specified in finnish
impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Day::Monday => "Ma",
            Day::Tuesday => "Ti",
            Day::Wednesday => "Ke",
            Day::Thursday => "To",
            Day::Friday => "Pe",
            Day::Saturday => "La",
            Day::Sunday => "Su",
        };

        write!(f, "{}", text)
    }
}

pub struct OverflowError;

#[derive(Hash, PartialEq, Eq, Default, Clone, Copy, Debug)]
pub struct Hour(u8);

impl TryFrom<u32> for Hour {
    type Error = OverflowError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 24 {
            Err(OverflowError)
        } else {
            Ok(Hour(value.try_into().unwrap()))
        }
    }
}

#[derive(Hash, PartialEq, Eq, Default, Debug)]
pub struct DayTime {
    pub day: Day,
    pub time: Hour,
}

impl Hour {
    pub const fn as_seconds(&self) -> u32 {
        self.0 as u32 * 3600
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:00:00", self.0)
    }
}

pub fn seconds_to_hhmmss(timestamp: u32) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        timestamp / 3600 % 60,
        timestamp / 60 % 60,
        timestamp % 60
    )
}
