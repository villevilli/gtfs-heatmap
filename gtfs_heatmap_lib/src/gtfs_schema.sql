DROP TABLE IF EXISTS agency;
CREATE TABLE agency (
  agency_id text UNIQUE NULL,
  agency_name text NOT NULL,
  agency_url text NOT NULL,
  agency_timezone text NOT NULL,
  agency_lang text NULL,
  agency_phone text NULL,
  agency_fare_url text NULL
);
DROP TABLE IF EXISTS stops;
CREATE TABLE stops (
  stop_id text PRIMARY KEY,
  stop_code text NULL,
  stop_name text NULL CHECK (
    location_type >= 0
    AND location_type <= 2
    AND stop_name IS NOT NULL
    OR location_type > 2
  ),
  stop_desc text NULL,
  stop_lat double precision NULL CHECK (
    location_type >= 0
    AND location_type <= 2
    AND stop_name IS NOT NULL
    OR location_type > 2
  ),
  stop_lon double precision NULL CHECK (
    location_type >= 0
    AND location_type <= 2
    AND stop_name IS NOT NULL
    OR location_type > 2
  ),
  zone_id text NULL,
  stop_url text NULL,
  location_type integer NULL CHECK (
    location_type >= 0
    AND location_type <= 4
  ),
  parent_station text NULL CHECK (
    location_type IS NULL
    OR location_type = 0
    OR location_type = 1
    AND parent_station IS NULL
    OR location_type >= 2
    AND location_type <= 4
    AND parent_station IS NOT NULL
  ),
  wheelchair_boarding integer NULL CHECK (
    wheelchair_boarding >= 0
    AND wheelchair_boarding <= 2
    OR wheelchair_boarding IS NULL
  ),
  platform_code text NULL,
  vehicle_type integer NULL
);
DROP TABLE IF EXISTS routes;
CREATE TABLE routes (
  route_id text PRIMARY KEY,
  agency_id text NULL REFERENCES agency(agency_id) ON DELETE CASCADE ON UPDATE CASCADE,
  route_short_name text NULL,
  route_long_name text NULL CHECK (
    route_short_name IS NOT NULL
    OR route_long_name IS NOT NULL
  ),
  route_desc text NULL,
  route_type integer NOT NULL,
  route_url text NULL
);
DROP TABLE IF EXISTS trips;
CREATE TABLE trips (
  route_id text NOT NULL REFERENCES routes ON DELETE CASCADE ON UPDATE CASCADE,
  service_id text NOT NULL,
  trip_id text NOT NULL PRIMARY KEY,
  trip_headsign text NULL,
  direction_id boolean NULL,
  shape_id text NULL,
  wheelchair_accessible integer NULL CHECK (
    wheelchair_accessible >= 0
    AND wheelchair_accessible <= 2
  ),
  bikes_allowed integer NULL CHECK (
    bikes_allowed >= 0
    AND bikes_allowed <= 2
  ),
  max_delay text NULL
);
DROP TABLE IF EXISTS stop_times;
CREATE TABLE stop_times (
  trip_id text NOT NULL REFERENCES trips ON DELETE CASCADE ON UPDATE CASCADE,
  arrival_time interval NULL,
  departure_time interval NOT NULL,
  stop_id text NOT NULL REFERENCES stops ON DELETE CASCADE ON UPDATE CASCADE,
  stop_sequence integer NOT NULL CHECK (stop_sequence >= 0),
  stop_headsign text NULL,
  pickup_type integer NOT NULL CHECK (
    pickup_type >= 0
    AND pickup_type <= 3
  ),
  drop_off_type integer NOT NULL CHECK (
    drop_off_type >= 0
    AND drop_off_type <= 3
  ),
  shape_dist_traveled double precision NULL CHECK (shape_dist_traveled >= 0.0),
  timepoint boolean NULL
);
DROP TABLE IF EXISTS calendar;
CREATE TABLE calendar (
  service_id text PRIMARY KEY,
  monday boolean NOT NULL,
  tuesday boolean NOT NULL,
  wednesday boolean NOT NULL,
  thursday boolean NOT NULL,
  friday boolean NOT NULL,
  saturday boolean NOT NULL,
  sunday boolean NOT NULL,
  start_date numeric(8) NOT NULL,
  end_date numeric(8) NOT NULL
);
DROP TABLE IF EXISTS calendar_dates;
CREATE TABLE calendar_dates (
  service_id text NOT NULL,
  date numeric(8) NOT NULL,
  exception_type integer NOT NULL CHECK (
    exception_type >= 1
    AND exception_type <= 2
  )
);
DROP TABLE IF EXISTS fare_attributes;
CREATE TABLE fare_attributes (
  fare_id text PRIMARY KEY,
  price double precision NOT NULL CHECK (price >= 0.0),
  currency_type text NOT NULL,
  payment_method boolean NOT NULL,
  transfers integer NULL CHECK (
    transfers >= 0
    AND transfers <= 5
  ),
  transfer_duration integer NULL CHECK (transfer_duration >= 0)
);
DROP TABLE IF EXISTS fare_rules;
CREATE TABLE fare_rules (
  fare_id text NOT NULL REFERENCES fare_attributes ON DELETE CASCADE ON UPDATE CASCADE,
  route_id text NULL REFERENCES routes ON DELETE CASCADE ON UPDATE CASCADE,
  origin_id text NULL,
  destination_id text NULL,
  contains_id text NULL
);
DROP TABLE IF EXISTS shapes;
CREATE TABLE shapes (
  shape_id text NOT NULL,
  shape_pt_lat double precision NOT NULL,
  shape_pt_lon double precision NOT NULL,
  shape_pt_sequence integer NOT NULL CHECK (shape_pt_sequence >= 0),
  shape_dist_traveled double precision NULL CHECK (shape_dist_traveled >= 0.0)
);
DROP TABLE IF EXISTS transfers;
CREATE TABLE transfers (
  from_stop_id text NOT NULL REFERENCES stops(stop_id) ON DELETE CASCADE ON UPDATE CASCADE,
  to_stop_id text NOT NULL REFERENCES stops(stop_id) ON DELETE CASCADE ON UPDATE CASCADE,
  transfer_type integer NOT NULL CHECK (
    transfer_type >= 0
    AND transfer_type <= 3
  ),
  min_transfer_time integer NULL CHECK (min_transfer_time >= 0)
);
DROP TABLE IF EXISTS feed_info;
CREATE TABLE feed_info (
  feed_publisher_name text NOT NULL,
  feed_publisher_url text NOT NULL,
  feed_lang text NULL,
  feed_start_date numeric(8) NULL,
  feed_end_date numeric(8) NULL,
  feed_version text NULL
);
DROP TABLE IF EXISTS translations;
CREATE TABLE translations (
  trans_id text NULL,
  lang text NULL,
  translation text NOT NULL
);