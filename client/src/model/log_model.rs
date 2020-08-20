use serde_json::Value;
use roguelike_common::*;
use yew::services::ConsoleService;

#[derive(Debug, PartialEq, Clone)]
pub struct MLog {
    pub logs: Logs,
}

impl MLog {
    pub fn new() -> Self {
         Self {
            logs: Vec::new(),
        }
    }

    pub fn set_logs(&mut self, data: Value) {
        self.logs = serde_json::from_value(data).unwrap();
        ConsoleService::info(&format!("LOGS {:?}", self.logs));
    }
}

