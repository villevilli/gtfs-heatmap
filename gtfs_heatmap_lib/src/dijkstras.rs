#![allow(unused)]

use rusqlite::Connection;

use crate::gtfs_types::{self, Stop};
use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

struct StopNode {
    stop_id: String,
    connections: Vec<StopConnection>,
    time_to: Option<u32>,
    is_visited: bool,
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

impl StopNode {
    fn gen_connections(&self, time: u32) -> Vec<StopConnection> {
        todo!()
    }
}

struct StopGraph {
    graph: Vec<StopNode>,
}

impl StopGraph {
    fn initialize(initial_stop_index: u32, stops: Vec<gtfs_types::Stop>) -> StopGraph {
        let mut graph: StopGraph = StopGraph { graph: Vec::new() };

        for (i, stop) in stops.iter().enumerate() {
            if i != initial_stop_index as usize {
                graph.graph.push(StopNode {
                    stop_id: stop.stop_id.clone(),
                    connections: Vec::new(),
                    time_to: None,
                    is_visited: false,
                })
            } else {
                graph.graph.insert(
                    0,
                    StopNode {
                        stop_id: stop.stop_id.clone(),
                        connections: Vec::new(),
                        time_to: Some(0),
                        is_visited: false,
                    },
                )
            }
        }

        todo!()
    }

    fn get_connections(&self, time: u32, connection: &Connection) -> Vec<StopConnection> {
        todo!()
    }

    fn to_lookup_table(&self) -> HashMap<String, u32> {
        todo!()
    }
}
