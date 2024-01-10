DROP SCHEMA IF EXISTS gtfs CASCADE;
CREATE SCHEMA gtfs;

DROP DOMAIN IF EXISTS wgs84_lat CASCADE;
CREATE DOMAIN wgs84_lat AS DOUBLE PRECISION CHECK(VALUE >= -90 AND VALUE <= 90);

DROP DOMAIN IF EXISTS wgs84_lon CASCADE;
CREATE DOMAIN wgs84_lon AS DOUBLE PRECISION CHECK(VALUE >= -180 AND VALUE <= 180);

DROP DOMAIN IF EXISTS gtfstime CASCADE;
CREATE DOMAIN gtfstime AS text CHECK(VALUE ~ '^[0-9]?[0-9]:[0-5][0-9]:[0-5][0-9]$');

CREATE TABLE agency
(
  agency_id         text UNIQUE NULL,
  agency_name       text NOT NULL,
  agency_url        text NOT NULL,
  agency_timezone   text NOT NULL,
  agency_lang       text NULL,
  agency_phone      text NULL
);

CREATE TABLE stops
(
  stop_id           text PRIMARY KEY,
  stop_code         text UNIQUE NULL,
  stop_name         text NOT NULL,
  stop_desc         text NULL,
  stop_lat          wgs84_lat NOT NULL,
  stop_lon          wgs84_lon NOT NULL,
  zone_id           text NULL,
  stop_url          text NULL,
  location_type     boolean NULL,
  parent_station    text NULL,
  wheelchair_boarding text NULL,
  stop_direction  text NULL
);

CREATE TABLE routes
(
  agency_id         text NULL,
  route_short_name  text NOT NULL,
  route_long_name   text NOT NULL,
  route_desc        text NULL,
  route_type        integer NULL,
  route_url         text NULL,
  route_color       text NULL,
  route_text_color  text NULL,
  route_bikes_allowed text NULL,
  route_id          text PRIMARY KEY
);

CREATE TABLE calendar
(
  service_id        text PRIMARY KEY,
  monday            boolean NOT NULL,
  tuesday           boolean NOT NULL,
  wednesday         boolean NOT NULL,
  thursday          boolean NOT NULL,
  friday            boolean NOT NULL,
  saturday          boolean NOT NULL,
  sunday            boolean NOT NULL,
  start_date        numeric(8) NOT NULL,
  end_date          numeric(8) NOT NULL
);

CREATE TABLE shapes
(
  shape_id          text,
  shape_pt_sequence integer NOT NULL,
  shape_dist_traveled double precision NULL,
  shape_pt_lat      wgs84_lat NOT NULL,
  shape_pt_lon      wgs84_lon NOT NULL
);

CREATE TABLE trips
(
  route_id          text NOT NULL,
  service_id        text NOT NULL,
  trip_short_name   text NULL,
  trip_headsign     text NULL,
  route_short_name  text NULL,
  direction_id      boolean NULL,
  block_id          text NULL,
  shape_id          text NULL,
  wheelchair_accessible text NULL,
  trip_bikes_allowed text NULL,
  trip_id           text NOT NULL PRIMARY KEY
);

CREATE TABLE stop_times
(
  trip_id           text NOT NULL,
  stop_sequence     integer NOT NULL,
  stop_id           text NOT NULL,
  arrival_time      interval NOT NULL,
  departure_time    interval NOT NULL,
  stop_headsign     text NULL,
  route_short_name  text NULL,
  pickup_type       integer NULL CHECK(pickup_type >= 0 and pickup_type <=3),
  drop_off_type     integer NULL CHECK(drop_off_type >= 0 and drop_off_type <=3),
  shape_dist_traveled double precision NULL
);

CREATE TABLE frequencies
(
  trip_id           text NOT NULL,
  start_time        interval NOT NULL,
  end_time          interval NOT NULL,
  headway_secs      integer NOT NULL,
  exact_times   text NULL
);

CREATE TABLE transfers
(
    from_stop_id  text NOT NULL,
    to_stop_id    text NOT NULL,
    transfer_type   integer NOT NULL
);

\copy agency from './data/agency.txt' with csv header
\copy stops from './data/stops.txt' with csv header
\copy routes from './data/routes.txt' with csv header
\copy calendar from './data/calendar.txt' with csv header
\copy shapes from './data/shapes.txt' with csv header
\copy trips from './data/trips.txt' with csv header
\copy stop_times from './data/stop_times.txt' with csv header
\copy frequencies from './data/frequencies.txt' with csv header
\copy transfers from './data/transfers.txt' with csv header