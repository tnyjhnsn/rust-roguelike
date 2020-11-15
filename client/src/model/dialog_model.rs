use serde_json::Value;
use yew::services::ConsoleService;

#[derive(Debug, PartialEq, Clone)]
pub struct MDialog {
    pub dialog: (i32, Vec<(i32, f32)>),
}

impl MDialog {
    pub fn new() -> Self {
         Self {
            dialog: (-1, Vec::new()),
        }
    }

    pub fn set_dialog(&mut self, data: Value) {
        self.dialog = serde_json::from_value(data).unwrap();
    }
}

