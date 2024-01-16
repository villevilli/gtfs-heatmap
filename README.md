# GTFS Heatmap

Contains a rust library to handle gtfs data for heatmapping purposes,
A rocket based backend server
And A svelte frontend

Import data by unpacking the gtfs zip file into the data folder and
running the following commands in the base direcory with gtfs3 cli utility connected to the database

```
.open gtfs_db.sqlite
.mode csv
.import --skip 1 data/agency.txt agency
.import --skip 1 data/stops.txt stops
.import --skip 1 data/routes.txt routes
.import --skip 1 data/trips.txt trips
.import --skip 1 data/stop_times.txt stop_times
.import --skip 1 data/calendar.txt calendar
.import --skip 1 data/calendar_dates.txt calendar_dates
.import --skip 1 data/shapes.txt shapes
.import --skip 1 data/transfers.txt transfers
.import --skip 1 data/feed_info.txt feed_info
.import --skip 1 data/translations.txt translations
```