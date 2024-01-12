use rusqlite::Row;
use serde::{Deserialize, Serialize};

use crate::coords::Coordinates;

#[derive(Serialize, Deserialize, Debug, Default)]
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
