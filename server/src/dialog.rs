use serde_json::Value;

type Items = Vec<(i32, f32)>;

pub struct VendorDialog {
    pub vendor: i32,
    pub items: Items,
}

impl VendorDialog {
    pub fn new() -> Self {
        Self {
            vendor: -1,
            items: Vec::new(),
        }
    }

    pub fn add_dialog(&mut self, vendor: i32, items: Items) {
        self.vendor = vendor;
        self.items = items;
    }

    pub fn get_dialog(&mut self) -> Option<Value> {
        if self.vendor != -1 {
            let dialog = serde_json::to_value((self.vendor, &self.items)).unwrap();
            self.vendor = -1;
            self.items.clear();
            Some(dialog)
        } else {
            None
        }
    }
}

