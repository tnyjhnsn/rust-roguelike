use std::collections::HashMap;
use super::{
    Map,
    Position,
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
    name: &'static str,
    start_position: Position,
    active_map_key: String,
    maps: HashMap<String, (Map, Exits)>,
}

impl Campaign {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        maps.entry(String::from("dm_gate")).or_insert((
            dm_gate::dwarven_mines_gate(),
            Exits::new(Position::new(15, 2), String::from("dm_hall"), Position::new(23, 48))
        ));
        maps.entry(String::from("dm_hall")).or_insert((
            dm_hall::dwarven_mines_hall(),
            Exits::new(Position::new(23, 48), String::from("dm_gate"), Position::new(15, 2))
        ));
        Self {
            name: "The Dwarven Mines",
            start_position: Position::new(15, 58),
            active_map_key: String::from("dm_gate"),
            maps,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_active_map(&mut self) -> &mut Map {
        &mut self.maps.get_mut(&self.active_map_key).unwrap().0
    }

    pub fn get_player_start(&self) -> Position {
        self.start_position
    }
}

