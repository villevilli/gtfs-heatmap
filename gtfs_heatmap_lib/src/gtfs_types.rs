use std::fmt::{self, format};
use std::str::FromStr;

use rusqlite::Row;
use serde::{de, Deserialize, Serialize};
use time::macros::format_description;

use speedate::Time;

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

impl StopTrip {
    pub fn try_from_row(row: &Row<'_>) -> Option<StopTrip> {
        let arrival_time_string: String = row.get(3).ok()?;
        let departure_time_string: String = row.get(3).ok()?;

        let mut stop_trip = StopTrip {
            stop_id: row.get(0).ok()?,
            trip_id: row.get(1).ok()?,
            stop_sequence: row.get(2).ok()?,
            arrival_time: parse_time(&arrival_time_string),
            departure_time: parse_time(&departure_time_string),
        };

        Some(stop_trip)
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
impl Stop {
    pub fn try_from_row(row: &Row<'_>) -> Option<Stop> {
        Some(Stop {
            stop_id: row.get(0).ok()?,
            stop_code: row.get(1).ok()?,
            stop_name: row.get(2).ok()?,
            stop_desc: row.get(3).ok()?,
            coordinates: Coordinates {
                latitude: row.get(4).ok()?,
                longitude: row.get(5).ok()?,
            },
            zone_id: row.get(6).ok()?,
            stop_url: row.get(7).ok()?,
            location_type: row.get(8).ok()?,
            parent_station: row.get(9).ok()?,
            wheelchair_boarding: row.get(10).ok()?,
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
