use super::map_model::*;
use super::log_model::*;
use super::inventory_model::*;
use super::dictionary::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MGame {
    pub title: String,
    pub stats: String,
    pub armour: String,
    pub inventory: MInventory,
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
            armour: String::from("Armour & Weapons"),
            inventory: MInventory::new(),
            minimap: String::from("Mini Map"),
            log: MLog::new(),
            map: MMap::new(),
            dict: Dictionary::new(),
        }
    }
}

