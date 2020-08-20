use chrono::prelude::*;
use serde_json::Value;
use roguelike_common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct MLog {
    pub logs: Vec<(DateTime<Local>, Logs)>,
}

impl MLog {
    pub fn new() -> Self {
         Self {
            logs: Vec::new(),
        }
    }

    pub fn set_logs(&mut self, data: Value) {
        let local: DateTime<Local> = Local::now();
        let logs: Logs = serde_json::from_value(data).unwrap();
        self.logs.push((local, logs));
    }
}

