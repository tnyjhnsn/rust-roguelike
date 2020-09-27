use std::collections::HashMap;
use super::{
    Map,
    Position,
    maps::*,
};

type Exits = HashMap<Position, (String, Position)>;

#[derive(Debug)]
pub struct Campaign {
    name: &'static str,
    start_position: Position,
    pub active_map: Map,
    active_map_key: String,
    maps: HashMap<String, Exits>,
    maps_2: HashMap<String, (Option<Map>, Exits)>,
}

impl Campaign {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        let mut maps_2 = HashMap::new();
        let mut exits = HashMap::new();

        exits.insert(Position::new(14, 1),
                (String::from("dm_hall"), Position::new(22, 48)));
        exits.insert(Position::new(15, 1),
                (String::from("dm_hall"), Position::new(23, 48)));
        exits.insert(Position::new(16, 1),
                (String::from("dm_hall"), Position::new(24, 48)));
        maps.insert(String::from("dm_gate"), exits.clone());
        maps_2.insert(String::from("dm_gate"), (None, exits));

        exits = HashMap::new();
        exits.insert(Position::new(22, 49),
                (String::from("dm_gate"), Position::new(14, 2)));
        exits.insert(Position::new(23, 49),
                (String::from("dm_gate"), Position::new(15, 2)));
        exits.insert(Position::new(24, 49),
                (String::from("dm_gate"), Position::new(16, 2)));
        exits.insert(Position::new(21, 0),
                (String::from("dm_forge"), Position::new(27, 48)));
        exits.insert(Position::new(22, 0),
                (String::from("dm_forge"), Position::new(27, 48)));
        exits.insert(Position::new(23, 0),
                (String::from("dm_forge"), Position::new(28, 48)));
        exits.insert(Position::new(24, 0),
                (String::from("dm_forge"), Position::new(29, 48)));
        exits.insert(Position::new(25, 0),
                (String::from("dm_forge"), Position::new(30, 48)));
        maps.insert(String::from("dm_hall"), exits.clone());
        maps_2.insert(String::from("dm_hall"), (None, exits));

        exits = HashMap::new();
        exits.insert(Position::new(27, 49),
                (String::from("dm_hall"), Position::new(21, 1)));
        exits.insert(Position::new(28, 49),
                (String::from("dm_hall"), Position::new(22, 1)));
        exits.insert(Position::new(29, 49),
                (String::from("dm_hall"), Position::new(23, 1)));
        exits.insert(Position::new(30, 49),
                (String::from("dm_hall"), Position::new(24, 1)));
        exits.insert(Position::new(14, 0),
                (String::from("dm_mine"), Position::new(14, 48)));
        exits.insert(Position::new(15, 0),
                (String::from("dm_mine"), Position::new(15, 48)));
        exits.insert(Position::new(16, 0),
                (String::from("dm_mine"), Position::new(16, 48)));
        exits.insert(Position::new(45, 0),
                (String::from("dm_mine"), Position::new(45, 48)));
        maps.insert(String::from("dm_forge"), exits.clone());
        maps_2.insert(String::from("dm_forge"), (None, exits));

        exits = HashMap::new();
        exits.insert(Position::new(14, 49),
                (String::from("dm_forge"), Position::new(14, 1)));
        exits.insert(Position::new(15, 49),
                (String::from("dm_forge"), Position::new(15, 1)));
        exits.insert(Position::new(16, 49),
                (String::from("dm_forge"), Position::new(16, 1)));
        exits.insert(Position::new(45, 49),
                (String::from("dm_forge"), Position::new(45, 1)));
        maps.insert(String::from("dm_mine"), exits.clone());
        maps_2.insert(String::from("dm_mine"), (None, exits));

        Self {
            name: "The Dwarven Mines",
            start_position: Position::new(15, 59),
            active_map: dm_gate::dwarven_mines_gate(),
            active_map_key: String::from("dm_gate"),
            maps,
            maps_2,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    // TEST
    pub fn store_map(&mut self, map: &Map) {
        for (key, val) in self.maps_2.iter_mut() {
            if key == &self.active_map_key {
                val.0 = Some(map.clone());
            }
        }
    }

    // TEST
    pub fn get_map(&self, key: String) -> Option<Map> {
        if let Some(map) = &self.maps_2.get(&key).unwrap().0 {
            Some(map.clone())
        } else {
            None
        }
    }

    pub fn get_active_map(&mut self) -> Map {
        self.active_map.clone()
    }

    pub fn get_player_start(&self) -> Position {
        self.start_position
    }

    pub fn exit_map(&mut self, ppos: Position) -> Position {
        let exits = self.maps.get(&self.active_map_key).unwrap();
        let (new_map_key, new_ppos) = &exits.get(&ppos).unwrap();

        match &new_map_key[..] {
            "dm_gate" => {
                self.active_map = dm_gate::dwarven_mines_gate();
                self.active_map_key = String::from("dm_gate");
            }
            "dm_hall" => {
                self.active_map = dm_hall::dwarven_mines_hall();
                self.active_map_key = String::from("dm_hall");
            }
            "dm_forge" => {
                self.active_map = dm_forge::dwarven_mines_forge();
                self.active_map_key = String::from("dm_forge");
            }
            "dm_mine" => {
                self.active_map = dm_mine::dwarven_mines_mine();
                self.active_map_key = String::from("dm_mine");
            }
            _ => {}
        }
        *new_ppos
    }
}

