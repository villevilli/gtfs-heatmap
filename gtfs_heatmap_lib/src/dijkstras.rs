#![allow(unused)]

use petgraph::{
    algo::dijkstra,
    graph::Node,
    graphmap::{self, GraphMap},
    Directed, Graph, Undirected,
};
use rusqlite::Connection;
use speedate::DateTime;

use crate::{
    get_next_stop, get_next_trips_by_time,
    gtfs_types::{self, Day, DayTime, Hour, Stop, StopTrip},
};
use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

type StopId = String;
type Distance = u32;
pub(crate) type TimeLookupTable = HashMap<StopId, u32>;

#[derive(Hash, Clone, Default, Debug)]
pub struct StopNode {
    pub stop_id: String,
    pub time_to: u32,
}

impl PartialEq for StopNode {
    fn eq(&self, other: &Self) -> bool {
        self.time_to == other.time_to
    }
}

impl Eq for StopNode {}

struct StopConnection {
    time: u32,
    node: Rc<StopNode>,
}

impl PartialOrd for StopNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.time_to.partial_cmp(&other.time_to)
    }
}

impl Ord for StopNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<StopTrip> for StopNode {
    fn from(value: StopTrip) -> Self {
        StopNode {
            stop_id: value.stop_id,
            time_to: value.departure_time,
        }
    }
}

fn gen_times(daytime: &DayTime, start_node: &StopNode, connection: &Connection) -> TimeLookupTable {
    let mut graph: BinaryHeap<StopNode> = BinaryHeap::new();
    let mut lookuptable = HashMap::new();

    graph.push(start_node.clone());

    while let Some(next_node) = graph.pop() {
        if lookuptable.contains_key(&next_node.stop_id) {
            continue;
        }
        if let Some(stops) = get_next_nodes(&next_node, connection, &daytime) {
            for stop in stops {
                graph.push(stop)
            }
        }

        lookuptable.insert(next_node.stop_id, next_node.time_to);
    }

    lookuptable
}

fn get_next_nodes(
    stop_node: &StopNode,
    connection: &Connection,
    daytime: &DayTime,
) -> Option<Vec<StopNode>> {
    let next_trips = get_next_trips_by_time(
        daytime.time.as_seconds() + stop_node.time_to,
        &daytime.day,
        stop_node.stop_id.clone(),
        connection,
    )
    .unwrap();

    let mut temp: Vec<StopNode> = next_trips
        .iter()
        .filter_map(|trip| get_next_stop(trip.trip_id.clone(), trip.stop_sequence, connection).ok())
        .map(|x| x.into())
        .map(|mut x: StopNode| {
            x.time_to = x.time_to - daytime.time.as_seconds();
            x
        })
        .collect();

    for stop_node in &mut temp {
        stop_node.time_to -= daytime.time.as_seconds();
    }

    Some(temp)
}

#[derive(Debug)]
pub struct StopMapCache {
    lookup_table: TimeLookupTable,
    daytime: DayTime,
    start_node: StopNode,
}

impl StopMapCache {
    //This is the most non rust thing to do but at this point I dont feel like giving a shit lmao
    pub fn new() -> StopMapCache {
        StopMapCache {
            lookup_table: HashMap::new(),
            daytime: DayTime::default(),
            start_node: StopNode::default(),
        }
    }

    pub fn get(&self) -> &TimeLookupTable {
        &self.lookup_table
    }

    pub fn is_current(&self, daytime: DayTime, start_node: StopNode) -> bool {
        self.daytime == daytime && self.start_node == start_node
    }

    pub fn get_if_current(
        &self,
        daytime: DayTime,
        start_node: StopNode,
    ) -> Option<&TimeLookupTable> {
        if self.is_current(daytime, start_node) {
            Some(self.get())
        } else {
            None
        }
    }

    pub fn update_lookup_table(
        &mut self,
        daytime: DayTime,
        start_node: StopNode,
        connection: &Connection,
    ) {
        self.lookup_table = gen_times(&daytime, &start_node, connection);
        self.daytime = daytime;
        self.start_node = start_node;
    }
}

impl Default for StopMapCache {
    fn default() -> Self {
        Self::new()
    }
}
