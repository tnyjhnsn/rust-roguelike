use std::collections::HashMap;
use super::{
    Map,
    Position,
    Point,
    maps::*,
};

type Exit = HashMap<Position, (String, Position)>;

#[derive(Debug)]
pub struct Exits {
    pub exits: Vec<Exit>,
}

impl Exits {
    pub fn new(exit_pos: Position, to_map: String, to_pos: Position) -> Self {
        let mut hm = HashMap::new();
        hm.entry(exit_pos).or_insert((to_map, to_pos));
        let exits = vec![hm];
        Self { exits }
    }
}

#[derive(Debug)]
pub struct Campaign {
    active_map: String,
    maps: HashMap<String, (Map, Point, Exits)>,
}

impl Campaign {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        maps.entry("dm_gate".to_string()).or_insert((
            dm_gate::dwarven_mines_gate(),
            Point::new(15, 58),
            Exits::new(Position::new(15, 2), "dm_hall".to_string(), Position::new(23, 48))
        ));
        maps.entry("dm_hall".to_string()).or_insert((
            dm_hall::dwarven_mines_hall(),
            Point::new(23, 48),
            Exits::new(Position::new(23, 48), "dm_gate".to_string(), Position::new(15, 2))
        ));
        Self {
            active_map: "dm_gate".to_string(),
            maps,
        }
    }

    pub fn get_active_map(&mut self) -> &mut Map {
        &mut self.maps.get_mut(&self.active_map).unwrap().0
    }

    pub fn get_player_start(&self) -> Point {
        self.maps.get(&self.active_map).unwrap().1
    }

}

