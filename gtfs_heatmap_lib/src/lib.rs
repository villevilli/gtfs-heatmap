pub mod coords;
pub mod dijkstras;
pub mod gtfs_types;
pub mod heatmap;

use coords::Coordinates;
use gtfs_types::{Day, StopTrip};
use rusqlite::{params, Connection};
use speedate::Duration;

#[derive(Debug)]
pub enum Error {
    ParseError,
    RusqliteError(rusqlite::Error),
}

use crate::gtfs_types::{seconds_to_hhmmss, Stop};

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

pub fn get_stop_connection(connection: &Connection) -> Result<Vec<Stop>, Error> {
    todo!()
}

pub fn get_nearby_stops(
    connection: &Connection,
    coords: Coordinates,
    search_box_distance: f64,
) -> Result<Vec<gtfs_types::Stop>, Error> {
    let mut query = connection
        .prepare_cached(
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

pub fn get_next_stop(
    trip_id: String,
    stop_sequence: u32,
    connection: &Connection,
) -> Result<StopTrip, Error> {
    let mut query = connection
        .prepare(
            "
            SELECT stop_id,
            trips.trip_id,
            MIN(stop_sequence),
            arrival_time,
            departure_time
        FROM stop_times
            JOIN trips ON stop_times.trip_id = trips.trip_id
        WHERE stop_sequence > (?2)
            AND stop_times.trip_id = (?1)
        GROUP BY stop_times.trip_id
        ORDER BY stop_sequence",
        )
        .map_err(|err| Error::RusqliteError(err))?;

    let mut binding = query
        .query(params![trip_id, stop_sequence])
        .map_err(|err| Error::RusqliteError(err))?;
    let row = binding
        .next()
        .map_err(|err| Error::RusqliteError(err))?
        .ok_or(Error::ParseError)?;

    Ok(StopTrip::try_from_row(row).ok_or(Error::ParseError)?)
}

pub fn get_next_trips_by_time(
    time: u32,
    day: &Day,
    stop_id: String,
    connection: &Connection,
) -> Result<Vec<StopTrip>, Error> {
    let mut query = connection
        .prepare_cached(
            "
        SELECT stop_id,
            trips.trip_id,
            stop_sequence,
            arrival_time,
            departure_time
        FROM stop_times
            JOIN trips ON stop_times.trip_id = trips.trip_id
        WHERE stop_id = (?1)
            AND departure_time >= time((?2))
            AND service_id LIKE (?3)
        GROUP BY trips.route_id
        ORDER BY departure_time;",
        )
        .map_err(|err| Error::RusqliteError(err))?;

    let mut rows = query
        .query(params![
            stop_id,
            seconds_to_hhmmss(time),
            "%_".to_string() + &day.to_string()
        ])
        .map_err(|err| Error::RusqliteError(err))?;

    let mut stop_trips = Vec::new();

    while let Some(row) = rows.next().map_err(|err| Error::RusqliteError(err))? {
        stop_trips.push(StopTrip::try_from_row(row).ok_or(Error::ParseError)?);
    }

    Ok(stop_trips)
}
