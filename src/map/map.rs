use std::collections::HashMap;
use std::hash::Hash;

pub struct Map<R,T> {
    pub map: HashMap<R, T>,
}

impl<R,T> Map<R,T> {
    pub fn new() -> Map<R,T> {
        Map {
            map: HashMap::new()
        }
    }

    pub fn push(&mut self, key: R, value: T) where R: Eq + Hash {
        self.map.insert(key, value);
    }
}

impl<R, T> ToString for Map<R, T> where R: ToString, T: ToString {
    fn to_string(&self) -> String {
        self.map
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .fold("".to_string(), |acc, (key, value)| format!("{}{} -> {}\n", acc, key, value))

    }
}
