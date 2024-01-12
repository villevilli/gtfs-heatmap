pub mod coords;
pub mod gtfs_types;
pub mod heatmap;

use coords::Coordinates;
use rusqlite::{params, Connection};

#[derive(Debug)]
pub enum Error {
    ParseError,
    RusqliteError(rusqlite::Error),
}

use crate::gtfs_types::Stop;

static DB_SCHEMA: &str = include_str!("gtfs_schema.sql");

pub fn initdb(location: &str) -> Result<Connection, Error> {
    let gtfs_db_connection = Connection::open(location).map_err(|err| Error::RusqliteError(err))?;
    gtfs_db_connection
        .execute_batch(DB_SCHEMA)
        .map_err(|err| Error::RusqliteError(err))?;
    Ok(gtfs_db_connection)
}

pub fn get_stops(connection: Connection) -> Result<Vec<gtfs_types::Stop>, Error> {
    let mut query = connection
        .prepare("SELECT * FROM stops")
        .map_err(|err| Error::RusqliteError(err))?;
    let mut rows = query.query([]).map_err(|err| Error::RusqliteError(err))?;
    let mut stops: Vec<Stop> = Vec::new();

    while let Some(row) = rows.next().map_err(|err| Error::RusqliteError(err))? {
        stops.push(Stop::try_from_row(row).ok_or(Error::ParseError)?);
    }

    Ok(stops)
}

pub fn get_nearby_stops(
    connection: &Connection,
    coords: Coordinates,
    search_box_distance: f64,
) -> Result<Vec<gtfs_types::Stop>, Error> {
    let mut query = connection
        .prepare(
            "SELECT * FROM stops WHERE 
            stop_lat < (?1)+(?3) AND stop_lat > (?1)-(?3) AND
            stop_lon < (?2)+(?3) AND stop_lon > (?2)-(?3)",
        )
        .map_err(|err| Error::RusqliteError(err))?;

    let mut rows = query
        .query(params![
            coords.latitude,
            coords.longitude,
            search_box_distance
        ])
        .map_err(|err| Error::RusqliteError(err))?;
    let mut stops: Vec<Stop> = Vec::new();

    while let Some(row) = rows.next().map_err(|err| Error::RusqliteError(err))? {
        stops.push(Stop::try_from_row(row).ok_or(Error::ParseError)?);
    }

    Ok(stops)
}
