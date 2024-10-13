use std::fmt::{self};
use std::str::FromStr;

use crate::coords::Coordinates;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(Debug, Default)]
pub struct StopTrip {
    pub stop_id: String,
    pub trip_id: String,
    pub stop_sequence: u32,
    pub arrival_time: u32,
    pub departure_time: u32,
}

pub fn parse_time(string: &str) -> u32 {
    let mut splits = string.split(":");

    let mut num = 0;

    num += splits.next().unwrap().parse::<u32>().unwrap() * 3600;
    num += splits.next().unwrap().parse::<u32>().unwrap() * 60;
    num += splits.next().unwrap().parse::<u32>().unwrap() * 1;

    num
}

#[derive(Debug)]
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

pub struct IntOutOfBounds;

impl TryFrom<u8> for Day {
    type Error = IntOutOfBounds;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Day::*;
        let day = match value {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            6 => Sunday,
            _ => return Err(IntOutOfBounds),
        };

        Ok(day)
    }
}

/*
impl Into<u8> for Day {
    fn into(self) -> u8 {
        use Day::*;
        match self {
            Monday => 0,
            Tuesday => 1,
            Wednesday => 2,
            Thursday => 3,
            Friday => 4,
            Saturday => 5,
            Sunday => 6,
        }
    }
}

*/

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
