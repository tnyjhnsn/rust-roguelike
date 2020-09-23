use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Dictionary {
    dict: HashMap<i32, (&'static str, &'static str)>,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut dict = HashMap::new();

        dict.insert(0, ("Hero", "player-m"));
        dict.insert(10, ("Carnivorous White Centipede", "white-centipede"));
        dict.insert(11, ("Giant Red Ant", "red-ant"));
        dict.insert(12, ("Scary Ghost", "ghost"));
        dict.insert(13, ("Shambling Grey Mould", "grey-mould"));
        dict.insert(2000, ("Health Potion", "health-potion"));
        dict.insert(2100, ("Magic Missiles Scroll", "scroll"));
        dict.insert(2101, ("Dragon Breath", "dragon-breath"));
        dict.insert(2102, ("Acid Rain", "acid-rain"));
        dict.insert(2103, ("Scroll of Confusion", "scroll-unlock"));
        dict.insert(3000, ("Dagger", "dagger"));
        dict.insert(3001, ("Short Sword", "short-sword"));
        dict.insert(3002, ("Long Sword", "long-sword"));
        dict.insert(3100, ("Shield", "shield"));
        dict.insert(5000, ("Chasm", ""));
        dict.insert(5001, ("Lava", ""));

        Self { dict }
    }

    pub fn get_name(&self, idx: i32) -> String {
        match self.dict.get(&idx) {
            Some(content) => (content.0).to_string(),
            None => String::from("Error getting content name"),
        }
    }

    pub fn get_css(&self, idx: i32) -> String {
        match self.dict.get(&idx) {
            Some(content) => (content.1).to_string(),
            None => String::from("Error getting css string"),
        }
    }
}
