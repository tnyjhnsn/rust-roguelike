use roguelike_common::{RandomNumberGenerator};

pub struct RandomEntry {
    code: i32,
    weight: i32,
}

impl RandomEntry {
    pub fn new(code: i32, weight: i32) -> Self {
        Self { code, weight }
    }
}

#[derive(Default)]
pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(mut self, code: i32, weight: i32) -> RandomTable {
        self.total_weight += weight;
        self.entries.push(RandomEntry::new(code, weight));
        self
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<i32> {
        if self.total_weight == 0 { return None; }
        let mut roll = rng.roll_dice(1, self.total_weight, -1);
        let mut idx: usize = 0;
        while roll > 0 {
            if roll < self.entries[idx].weight {
                return Some(self.entries[idx].code);
            }
            roll -= self.entries[idx].weight;
            idx += 1;
        }
        None
    }
}
