#![feature(async_closure)]

pub mod coords;
pub mod dijkstras;
pub mod gtfs_types;
pub mod heatmap;

use std::os::linux::raw::stat;

use coords::Coordinates;
use deadpool_postgres::tokio_postgres::{self, Client};
use futures::executor;
use gtfs_types::{seconds_to_hhmmss, Day, StopTrip};

#[derive(Debug)]
pub enum Error {
    ParseError,
    PostgresError(tokio_postgres::Error),
}

impl From<tokio_postgres::Error> for Error {
    fn from(value: tokio_postgres::Error) -> Self {
        Self::PostgresError(value)
    }
}

use crate::gtfs_types::Stop;

static DB_SCHEMA: &str = include_str!("gtfs_schema.sql");

// pub fn initdb(location: &str) -> Result<Client, Error> {
//     let mut gtfs_db_connection =
//         Client::connect(location, NoTls).map_err(|err| Error::PostgresError(err))?;
//     gtfs_db_connection
//         .batch_execute(DB_SCHEMA)
//         .map_err(|err| Error::PostgresError(err))?;
//     Ok(gtfs_db_connection)
// }

pub async fn get_stops(client: &Client) -> Result<Vec<gtfs_types::Stop>, Error> {
    let rows = client
        .query("SELECT * FROM stops", &[])
        .await
        .map_err(|err| Error::PostgresError(err))?;

    rows.iter()
        .map(|row| row.try_into().map_err(|err| Error::PostgresError(err)))
        .collect()
}

pub fn get_stop_connection(client: &Client) -> Result<Vec<Stop>, Error> {
    todo!()
}

pub async fn get_nearby_stops(
    client: &Client,
    coords: &Coordinates,
    search_box_distance: &f64,
) -> Result<Vec<gtfs_types::Stop>, Error> {
    let mut statement = client
        .prepare(
            "SELECT * FROM stops WHERE 
            stop_lat < ($1)+($3) AND stop_lat > ($1)-($3) AND
            stop_lon < ($2)+($3) AND stop_lon > ($2)-($3)",
        )
        .await
        .map_err(|err| Error::PostgresError(err))?;

    let rows = client
        .query(
            &statement,
            &[&coords.latitude, &coords.longitude, &search_box_distance],
        )
        .await
        .map_err(|x| Error::PostgresError(x))?;
    rows.iter()
        .map(|row| Stop::try_from(row).map_err(|err| Error::PostgresError(err)))
        .collect()
}

pub fn get_next_stop_sync(
    trip_id: &String,
    stop_sequence: &u32,
    client: &Client,
) -> Result<StopTrip, Error> {
    let next_stop = get_next_stop(trip_id, stop_sequence, client);
    executor::block_on(next_stop)
}

pub async fn get_next_stop(
    trip_id: &String,
    stop_sequence: &u32,
    client: &Client,
) -> Result<StopTrip, Error> {
    let statement = client
        .prepare(
            "
        SELECT stop_id,
            trips.trip_id,
            stop_sequence,
            EXTRACT(
                epoch
                FROM arrival_time
            ) as arrival_time,
            EXTRACT(
                epoch
                FROM departure_time
            ) as departure_time
        FROM stop_times
            JOIN trips ON stop_times.trip_id = trips.trip_id
        WHERE stop_sequence > ($2)
            AND stop_times.trip_id = ($1)
        ORDER BY stop_sequence
        LIMIT 1;
        ",
        )
        .await
        .map_err(|err| Error::PostgresError(err))?;

    let row = client
        .query_one(&statement, &[trip_id, stop_sequence])
        .await?;

    Ok(StopTrip::try_from(&row)?)
}

pub async fn get_next_trips_by_time(
    time: u32,
    day: &Day,
    stop_id: &String,
    client: &Client,
) -> Result<Vec<StopTrip>, Error> {
    let statement = client
        .prepare(
            "
        SELECT DISTINCT ON (trips.route_id) stop_id,
            trips.trip_id,
            stop_sequence,
            EXTRACT(
                epoch
                FROM arrival_time
            ) as arrival_time,
            EXTRACT(
                epoch
                FROM departure_time
            ) as departure_time
        FROM stop_times
            JOIN trips ON stop_times.trip_id = trips.trip_id
        WHERE stop_id = ($1)
            AND departure_time >= make_interval(0, 0, 0, 0, 0, 0, $2)
            AND service_id SIMILAR TO ($3)
        ORDER BY departure_time;",
        )
        .await?;

    let rows = client
        .query(
            &statement,
            &[
                stop_id,
                &seconds_to_hhmmss(time),
                &("%_".to_string() + &day.to_string()),
            ],
        )
        .await?;

    rows.iter()
        .map(|x| StopTrip::try_from(x).map_err(|err| Error::PostgresError(err)))
        .collect()
}
