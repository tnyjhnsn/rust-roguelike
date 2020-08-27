use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Dictionary {
    dict: HashMap<i32, (String, String)>,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut dict = HashMap::new();

        dict.insert(0, (String::from("Hero"), String::from("player-m")));
        dict.insert(10, (String::from("Carnivorous White Centipede"), String::from("white-centipede")));
        dict.insert(11, (String::from("Giant Red Ant"), String::from("red-ant")));
        dict.insert(12, (String::from("Scary Ghost"), String::from("ghost")));
        dict.insert(13, (String::from("Shambling Grey Mould"), String::from("grey-mould")));
        dict.insert(2000, (String::from("Health Potion"), String::from("health-potion")));

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
