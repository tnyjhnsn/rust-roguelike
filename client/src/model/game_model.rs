use super::map_model::*;
use super::log_model::*;
use super::inventory_model::*;
use super::armour_model::*;
use super::stats_model::*;
use super::dictionary::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MGame {
    pub title: String,
    pub stats: MStats,
    pub armour: MArmour,
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
            stats: MStats::new(),
            armour: MArmour::new(),
            inventory: MInventory::new(),
            minimap: String::from("Mini Map"),
            log: MLog::new(),
            map: MMap::new(),
            dict: Dictionary::new(),
        }
    }
}

