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
WHERE stop_id = '2132202'
    AND departure_time > make_interval(0, 0, 0, 0, 0, 0, 28000)
    AND service_id SIMILAR TO '%_Ti'
ORDER BY trips.route_id,
    departure_time;
--@block
EXPLAIN ANALYZE
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
WHERE stop_sequence > 33
    AND stop_times.trip_id = '2114_20240109_Ti_1_0720'
ORDER BY stop_sequence
LIMIT 1;
--@block
SELECT *
FROM stops
WHERE stop_code LIKE 'E1802';