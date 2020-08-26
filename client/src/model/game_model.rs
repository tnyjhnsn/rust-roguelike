use super::map_model::*;
use super::log_model::*;
use std::collections::HashMap;
use roguelike_common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MGame {
    pub title: String,
    pub stats: String,
    pub body: String,
    pub inventory: String,
    pub minimap: String,
    pub log: MLog,
    pub map: MMap,
    pub dict: Dictionary,
}

impl MGame {
    pub fn new() -> Self {
        Self {
            title: String::from("Rogue"),
            stats: String::from("Stats"),
            body: String::from("Body"),
            inventory: String::from("Inventory"),
            minimap: String::from("Mini Map"),
            log: MLog::new(),
            map: MMap::new(),
            dict: create_dict(),
        }
    }
}

// temporary hard coded for testing
fn create_dict() -> Dictionary {
    let mut dict = HashMap::new();
    dict.insert(0, (String::from("You"), String::from("player-m")));
    dict.insert(10, (String::from("Carnivorous White Centipede"), String::from("white-centipede")));
    dict.insert(11, (String::from("Giant Red Ant"), String::from("red-ant")));
    dict.insert(12, (String::from("Scary Ghost"), String::from("ghost")));
    dict.insert(13, (String::from("Shambling Grey Mould"), String::from("grey-mould")));
    dict.insert(2000, (String::from("Health Potion"), String::from("health-potion")));
    dict
}
