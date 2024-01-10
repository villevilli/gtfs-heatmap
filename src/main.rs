use rusqlite::Connection;

const DB_LOCATION: &str = "gtfs_db.sqlite";
static DB_SCHEMA: &str = include_str!("gtfs_schema.sql");

fn main() {
    let gtfs_db_connection = Connection::open(DB_LOCATION).unwrap();
    let err = gtfs_db_connection
        .execute_batch(DB_SCHEMA)
        .expect_err("LOL WORKS");

    println!("{}", err.to_string());
}
