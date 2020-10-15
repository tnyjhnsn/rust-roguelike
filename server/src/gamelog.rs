use serde_json::Value;
use roguelike_common::*;

pub struct GameLog {
    pub logs: Vec<Vec<i32>>,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![vec![LogType::System as i32, 0]],
        }
    }

    pub fn add_log(&mut self, log: Vec<i32>) {
        self.logs.push(log);
    }

    pub fn get_logs(&mut self) -> Option<Value> {
        if self.logs.len() > 0 {
            let logs = serde_json::to_value(&self.logs).unwrap();
            self.logs.clear();
            Some(logs)
        } else {
            None
        }
    }
}

