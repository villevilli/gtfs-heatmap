pub mod gtfs_types;

use std::vec;

use rusqlite::{Connection, Error};

use crate::gtfs_types::Stop;

static DB_SCHEMA: &str = include_str!("gtfs_schema.sql");

pub fn initdb(location: &str) -> Result<Connection, Error> {
    let gtfs_db_connection = Connection::open(location)?;
    gtfs_db_connection.execute_batch(DB_SCHEMA)?;
    Ok(gtfs_db_connection)
}

pub fn get_stops(connection: Connection) -> Result<Vec<gtfs_types::Stop>, Error> {
    let mut query = connection.prepare("SELECT * FROM stops")?;
    let mut rows = query.query([])?;
    let mut stops: Vec<Stop> = Vec::new();

    while let Some(row) = rows.next()? {
        stops.push(Stop {
            stop_id: row.get(0)?,
            stop_code: row.get(1)?,
            stop_name: row.get(2)?,
            stop_desc: row.get(3)?,
            stop_lat: row.get(4)?,
            stop_lon: row.get(5)?,
            zone_id: row.get(6)?,
            stop_url: row.get(7)?,
            location_type: row.get(8)?,
            parent_station: row.get(9)?,
            wheelchair_boarding: row.get(10)?,
        })
    }

    Ok(stops)
}
