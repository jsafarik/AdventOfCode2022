use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use regex::Regex;
use crate::graph::edge::Edge;

#[derive(Eq, Clone)]
pub struct Node {
    name: String,
    weight: usize,
    edges: HashSet<Edge>,
}

impl Node {
    pub fn parse(line: &str) -> Node {
        let capture = Regex::new(
            "Valve (?P<origin>\\w+) has flow rate=(?P<flow>\\d+); tunnels? leads? to valves? (?P<destination>.*)"
        ).unwrap().captures(line).unwrap();
        Node {
            name: capture.name("origin").unwrap().as_str().to_string(),
            weight: capture.name("flow").unwrap().as_str().parse().unwrap(),
            edges: HashSet::from_iter(
                capture
                    .name("destination")
                    .unwrap()
                    .as_str()
                    .split(",")
                    .map(|target| Edge::new(1, target.trim().to_string()))
            ),
        }
    }

    pub fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn insert_edge(&mut self, target: String, weight: usize) {
        self.edges.insert(Edge::new(weight, target));
    }

    pub fn remove_edge(&mut self, target: String) {
        let target = self.get_edge_by_target_name(target);
        self.edges.remove(&target);
    }

    fn get_edge_by_target_name(&self, target_name: String) -> Edge {
        self.edges.iter().find(|edge| edge.get_target_node() == target_name).unwrap().clone()
    }

    pub fn get_graph_csacademy_format(&self) -> String {
        let mut output = String::new();
        for target in &self.edges {
            output.push_str(format!("{} {} {}\n", self.name, target.get_target_node(), target.get_weight()).as_str());
        }
        output
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
