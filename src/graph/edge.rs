use std::hash::{Hash, Hasher};

#[derive(Eq, Clone)]
pub struct Edge {
    weight: usize,
    target_node: String,
}

impl Edge {
    pub fn new(weight: usize, target_node: String) -> Self {
        Edge { weight, target_node }
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn get_target_node(&self) -> String {
        self.target_node.clone()
    }
}

impl PartialEq<Self> for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.target_node == other.target_node
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.target_node.hash(state);
    }
}
