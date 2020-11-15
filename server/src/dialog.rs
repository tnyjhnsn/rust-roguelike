use serde_json::Value;

type Items = Vec<(i32, f32)>;

pub struct VendorDialog {
    pub dialog: (i32, Items),
}

impl VendorDialog {
    pub fn new() -> Self {
        Self {
            dialog: (-1, Vec::new()),
        }
    }

    pub fn add_dialog(&mut self, dialog: (i32, Items)) {
        self.dialog = dialog;
    }

    pub fn get_dialog(&mut self) -> Option<Value> {
        if self.dialog.0 != -1 {
            let dialog = serde_json::to_value(&self.dialog).unwrap();
            self.dialog = (-1, Vec::new());
            Some(dialog)
        } else {
            None
        }
    }
}

