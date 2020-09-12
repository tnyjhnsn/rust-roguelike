use serde_json::Value;
use roguelike_common::*;

pub struct GameLog {
    pub logs: Vec<Vec<i32>>,
    pub has_log: bool,
}

impl GameLog {
    pub fn new() -> Self {
        Self {
            logs: vec![vec![LogType::System as i32, 0]],
            has_log: true,
        }
    }

    pub fn add_log(&mut self, log: Vec<i32>) {
        self.logs.push(log);
        self.has_log = true;
    }

    pub fn get_logs(&mut self) -> Option<Value> {
        if self.has_log {
            let logs = serde_json::to_value(&self.logs).unwrap();
            self.logs.clear();
            self.has_log = false;
            Some(logs)
        } else {
            None
        }
    }
}

