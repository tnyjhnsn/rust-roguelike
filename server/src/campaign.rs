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
    active_map_key: String,
    maps: HashMap<String, (bool, Exits)>,
}

impl Campaign {
    pub fn new() -> Self {
        let mut maps = HashMap::new();
        let mut exits = HashMap::new();

        exits.insert(Position::new(14, 1),
                (String::from("dm_hall"), Position::new(22, 48)));
        exits.insert(Position::new(15, 1),
                (String::from("dm_hall"), Position::new(23, 48)));
        exits.insert(Position::new(16, 1),
                (String::from("dm_hall"), Position::new(24, 48)));
        maps.insert(String::from("dm_gate"), (false, exits));

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
        maps.insert(String::from("dm_hall"), (false, exits));

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
        maps.insert(String::from("dm_forge"), (false, exits));

        exits = HashMap::new();
        exits.insert(Position::new(14, 49),
                (String::from("dm_forge"), Position::new(14, 1)));
        exits.insert(Position::new(15, 49),
                (String::from("dm_forge"), Position::new(15, 1)));
        exits.insert(Position::new(16, 49),
                (String::from("dm_forge"), Position::new(16, 1)));
        exits.insert(Position::new(45, 49),
                (String::from("dm_forge"), Position::new(45, 1)));
        exits.insert(Position::new(7, 0),
                (String::from("dm_mountain"), Position::new(19, 48)));
        exits.insert(Position::new(8, 0),
                (String::from("dm_mountain"), Position::new(20, 48)));
        exits.insert(Position::new(9, 0),
                (String::from("dm_mountain"), Position::new(21, 48)));
        maps.insert(String::from("dm_mine"), (false, exits));

        exits = HashMap::new();
        exits.insert(Position::new(19, 49),
                (String::from("dm_mine"), Position::new(7, 1)));
        exits.insert(Position::new(20, 49),
                (String::from("dm_mine"), Position::new(8, 1)));
        exits.insert(Position::new(21, 49),
                (String::from("dm_mine"), Position::new(9, 1)));
        exits.insert(Position::new(22, 49),
                (String::from("dm_mine"), Position::new(9, 1)));
        maps.insert(String::from("dm_mountain"), (false, exits));

        exits = HashMap::new();
        exits.insert(Position::new(0, 0),
                (String::from("dm_gate"), Position::new(15, 59)));
        maps.insert(String::from("no_map"), (false, exits));

        Self {
            name: "The Dwarven Mines",
            active_map_key: String::from("no_map"),
            maps,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_visited(&mut self) {
        for (key, val) in self.maps.iter_mut() {
            if key == &self.active_map_key {
                val.0 = true;
            }
        }
    }

    pub fn get_visited(&self) -> bool {
        self.maps.get(&self.active_map_key).unwrap().0
    }

    pub fn create_map_from_exit(&mut self, ppos: Position) -> (Map, Position, bool) {
        self.set_visited();

        let exits = self.maps.get(&self.active_map_key).unwrap();
        let (new_map_key, new_ppos) = exits.1.get(&ppos).unwrap();

        let map = match &new_map_key[..] {
            "dm_hall" => dm_hall::dwarven_mines_hall(),
            "dm_forge" => dm_forge::dwarven_mines_forge(),
            "dm_mine" => dm_mine::dwarven_mines_mine(),
            "dm_mountain" => dm_mountain::dwarven_mines_mountain(),
            _ => dm_gate::dwarven_mines_gate(),
        };
        self.active_map_key = String::from(map.key);
        let visited = self.get_visited();
        (map, *new_ppos, visited)
    }
}

