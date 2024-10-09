use std::{
    any::Any,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    sync::{Arc, RwLock},
};

use time::{Duration, OffsetDateTime};

use super::{Error, GtfsGraph, Stop};

struct StopWithDuration {
    stop: Arc<RwLock<Stop>>,
    duration: Duration,
}

//Ord implemented as reverse, so we get a min heap from rust BinaryHeap
impl Ord for StopWithDuration {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.duration.cmp(&other.duration) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less,
        }
    }
}

impl PartialOrd for StopWithDuration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for StopWithDuration {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
    }
}

impl Eq for StopWithDuration {}

impl GtfsGraph {
    pub fn dijkstras(
        &self,
        start_id: &str,
        start_time: OffsetDateTime,
    ) -> Result<HashMap<String, Duration>, Error> {
        let mut queue: BinaryHeap<StopWithDuration> = BinaryHeap::new();
        let mut times: HashMap<String, Duration> = HashMap::new();

        queue.push(StopWithDuration {
            stop: self
                .get_stop(start_id)
                .ok_or(Error::MissingStop(start_id.to_string()))?,
            duration: Duration::ZERO,
        });

        while let Some(stop_with_duration) = queue.pop() {
            let stop = stop_with_duration.stop.read()?;

            if times.contains_key(&stop.id) {
                continue;
            }

            let unvisited_edges = stop
                .edges
                .iter()
                .filter(|edge| !times.contains_key(&edge.connected_stop.read().unwrap().id));

            let mut times_until_stops: HashMap<String, OffsetDateTime> = HashMap::new();

            for edge in unvisited_edges {
                let id = &edge.connected_stop.read()?.id;

                let mut time =
                    edge.departure_datetime((start_time + stop_with_duration.duration).date());

                if time < start_time + stop_with_duration.duration {
                    continue;
                }

                if times_until_stops.contains_key(id) {
                    let val = times_until_stops
                        .get_mut(id)
                        .expect("We just checked if id exsist in map");

                    if &mut time < val {
                        times_until_stops.insert(id.clone(), time);
                    }
                } else {
                    times_until_stops.insert(id.clone(), time);
                }
            }

            for (id, mut time_h) in times_until_stops.drain() {
                queue.push(StopWithDuration {
                    stop: self.get_stop(&id).expect("Stop id should be valid"),
                    duration: time_h - (start_time + stop_with_duration.duration),
                });
            }

            times.insert(stop.id.clone(), stop_with_duration.duration);
        }

        Ok(times)
    }
}
