use super::map::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub title: String,
    pub stats: String,
    pub body: String,
    pub inventory: String,
    pub minimap: String,
    pub log: String,
    pub map: Map,
}

impl Game {
    pub fn new() -> Self {
        Game {
            title: String::from("Rogue"),
            stats: String::from("Stats"),
            body: String::from("Body"),
            inventory: String::from("Inventory"),
            minimap: String::from("Mini Map"),
            log: String::from("Log"),
            map: Map::new(),
        }
    }
}

