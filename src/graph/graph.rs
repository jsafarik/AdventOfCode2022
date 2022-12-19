use std::collections::HashSet;
use crate::graph::node::Node;

pub struct Graph {
    nodes: HashSet<Node>,
    start: String,
    cached: bool,
}

impl Graph {
    pub fn parse(input: &str, start: &str) -> Graph {
        Graph {
            nodes: HashSet::from_iter(input.lines().map(|line| Node::parse(line))),
            start: start.to_string(),
            cached: false,
        }
    }

    /// Replace edges leading to nodes with weight = 0 with edges leading to
    /// non-zero children
    pub fn eliminate_edges_to_nodes_weighting_zero(&mut self) {
        loop {
            let mut made_alteration = false;

            for mut current_node in self.nodes.clone() {
                for target in current_node.get_edges().clone() {
                    let target_node = self.nodes.iter().find(|node| node.get_name() == target.get_target_node()).unwrap();

                    if target_node.get_weight() == 0 && target_node.get_name() != self.start {
                        made_alteration = true;
                        current_node.remove_edge(target.get_target_node());

                        for targets_target in target_node.get_edges() {
                            if *targets_target.get_target_node() == current_node.get_name() {
                                continue;
                            }
                            current_node.insert_edge(targets_target.get_target_node(), targets_target.get_weight() + target.get_weight());
                        }
                    }
                }

                self.nodes.replace(current_node);
            }

            if !made_alteration {
                break;
            }
        }
    }

    /// Remove all edges that can't be reached
    pub fn remove_unreachable_nodes(&mut self) {
        let mut to_drop = vec![];
        for current in &self.nodes {
            let mut drop = true;
            for other in &self.nodes {
                if current.get_name() == other.get_name() {
                    continue;
                } else if other.get_edges().iter().find(|edge| edge.get_target_node() == current.get_name()).is_some() {
                    drop = false;
                }
            }
            if drop {
                to_drop.push(current.clone());
            }
        }

        for node in to_drop {
            self.nodes.remove(&node);
        }
    }

    /// Create paths from all edges to all edges
    pub fn cache_all_edges(&mut self) {
        for mut a in self.nodes.clone() {
            for b in self.nodes.clone() {
                if a == b {
                    continue;
                }

                if a.get_edges().iter().find(|edge| edge.get_target_node() == b.get_name()).is_some() {
                    continue;
                }

                a.insert_edge(b.get_name(), self.get_shortest_path(a.get_name().as_str(), b.get_name().as_str()));
                self.nodes.replace(a.clone());
            }
        }
    }

    pub fn get_shortest_path(&self, a: &str, b: &str) -> usize {
        if self.cached {
            return self.nodes
                .iter()
                .find(|node| node.get_name().as_str() == a)
                .unwrap()
                .get_edges()
                .iter()
                .find(|edge| edge.get_target_node().as_str() == b)
                .unwrap()
                .get_weight();
        }
        let mut todo = vec![(a.to_string(), 0)];
        let mut done: Vec<String> = vec![];
        let mut path = usize::MAX;
        while !todo.is_empty() {
            let (current, shortest_path) = todo.remove(0);
            for (target_name, target_path_weight) in self.nodes
                .iter()
                .find(|node| node.get_name().as_str() == current)
                .unwrap()
                .get_edges()
                .iter()
                .map(|edge| (edge.get_target_node(), edge.get_weight()).clone())
                .clone() {
                let tmp_path = shortest_path + target_path_weight;
                if target_name == b && tmp_path < path {
                    path = tmp_path;
                } else if !done.contains(&target_name) {
                    todo.push((target_name, tmp_path));
                }
            }
            done.push(current);
        }

        if path == usize::MAX {
            panic!("No path for {} and {} found!", a, b);
        }
        path
    }

    pub fn print_graph_csacademy_format(&self) {
        for node in self.nodes.iter() {
            println!("{}", node.get_graph_csacademy_format());
        }
    }

    pub fn get_nodes(&self) -> HashSet<Node> {
        self.nodes.clone()
    }
}
