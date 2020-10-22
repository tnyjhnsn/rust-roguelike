use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct MStats {
    pub health: (i32, i32),
    pub mana: (i32, i32),
    pub might: (i32, i32, i32),
    pub fitness: (i32, i32, i32),
    pub quickness: (i32, i32, i32),
    pub intelligence: (i32, i32, i32),
}

impl MStats {
    pub fn new() -> Self {
         Self {
            health: (0, 0),
            mana: (0, 0),
            might: (0, 0, 0),
            fitness: (0, 0, 0),
            quickness: (0, 0, 0),
            intelligence: (0, 0, 0),
        }
    }

    pub fn set_combat(&mut self, data: Value) {
        let stats: Vec<(i32, i32)> = serde_json::from_value(data).unwrap();
        self.health = stats[0];
        self.mana = stats[1];
    }

    pub fn set_attributes(&mut self, data: Value) {
        let stats: Vec<(i32, i32, i32)> = serde_json::from_value(data).unwrap();
        self.might = stats[0];
        self.fitness = stats[1];
        self.quickness = stats[2];
        self.intelligence = stats[3];
    }
}

