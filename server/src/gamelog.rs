use serde_json::json;
use roguelike_common::*;
use std::collections::HashMap;

pub struct GameLog {
    pub logs: Vec<Vec<i32>>,
    pub has_log: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![vec![0,0]],
            has_log: true,
        }
    }

    pub fn add_log(&mut self, log: Vec<i32>) {
        self.logs.push(log);
        self.has_log = true;
    }

    pub fn draw_gamelog(&mut self) -> Option<String> {
        if self.has_log == true {
            let mut map = HashMap::new();
            map.entry(String::from("LOG")).or_insert(&self.logs);
            let gm = GameMsg {
                data: json!(map),
            };
            let s = serde_json::to_string(&gm).unwrap();
            //println!("GAMELOG {}", s);
            self.logs.clear();
            self.has_log = false;
            Some(s)
        } else {
            None
        }
    }
}

