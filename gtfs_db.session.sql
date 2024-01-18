SELECT stop_id,
    trips.trip_id,
    stop_sequence,
    arrival_time,
    departure_time
FROM stop_times
    JOIN trips ON stop_times.trip_id = trips.trip_id
WHERE stop_id = 1361103
    AND departure_time >= (3600)
    AND service_id LIKE "%_Ma"
GROUP BY trips.route_id,
    stop_sequence
ORDER BY departure_time;
--@block
SELECT stop_id,
    trips.trip_id,
    stop_sequence,
    arrival_time,
    departure_time
FROM stop_times
    JOIN trips ON stop_times.trip_id = trips.trip_id
WHERE stop_sequence > 16
    AND stop_times.trip_id = "1055_20240109_Ma_1_1134"
GROUP BY stop_times.trip_id
ORDER BY stop_sequence;
--@block
SELECT unixepoch(time("20:00:00"));