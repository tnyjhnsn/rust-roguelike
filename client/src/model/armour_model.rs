use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct MArmour {
    pub items: Vec<(i32, i32)>,
}

impl MArmour {
    pub fn new() -> Self {
         Self {
            items: Vec::new(),
        }
    }

    pub fn set_items(&mut self, data: Value) {
        self.items = serde_json::from_value(data).unwrap();
    }
}

