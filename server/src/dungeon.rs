use std::collections::HashMap;
use super::{Map};

#[derive(Default, Clone)]
pub struct Dungeon {
    maps: HashMap<i32, Map>,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    pub fn store_map(&mut self, map: &Map) {
        self.maps.insert(map.depth, map.clone());
    }

    pub fn get_map(&self, depth: i32) -> Option<Map> {
        if self.maps.contains_key(&depth) {
            let map = self.maps[&depth].clone();
            Some(map)
        } else {
            None
        }
    }
}
