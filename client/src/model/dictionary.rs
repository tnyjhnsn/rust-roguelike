use std::collections::HashMap;
use roguelike_common::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Dictionary {
    dict: HashMap<i32, (&'static str, &'static str)>,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut dict = HashMap::new();

        dict.insert(0, ("Hero", "player-m"));
        dict.insert(MOB_WHITE_CENTIPEDE, ("Carnivorous White Centipede", "white-centipede"));
        dict.insert(MOB_RED_ANT, ("Giant Red Ant", "red-ant"));
        dict.insert(MOB_GHOST, ("Scary Ghost", "ghost"));
        dict.insert(MOB_GREY_MOULD, ("Shambling Grey Mould", "grey-mould"));
        dict.insert(MOB_KOBOLD, ("Annoying Kobold", "kobold"));
        dict.insert(MOB_THIEF, ("Sneaky Thief", "Thief"));
        dict.insert(ITEM_HEALTH_POTION, ("Health Potion", "health-potion"));
        dict.insert(ITEM_MAGIC_MISSILE, ("Magic Missiles Scroll", "scroll"));
        dict.insert(ITEM_DRAGON_BREATH, ("Dragon Breath", "dragon-breath"));
        dict.insert(ITEM_ACID_RAIN, ("Acid Rain", "acid-rain"));
        dict.insert(ITEM_CONFUSION_SCROLL, ("Scroll of Confusion", "scroll-unlock"));
        dict.insert(WEAP_DAGGER, ("Dagger", "dagger"));
        dict.insert(WEAP_RUSTY_SWORD, ("Rusty Sword", "long-sword"));
        dict.insert(WEAP_LONG_SWORD, ("Long Sword", "long-sword"));
        dict.insert(WEAP_SHIELD, ("Shield", "shield"));
        dict.insert(TRAP_CHASM, ("Chasm", ""));
        dict.insert(TRAP_LAVA, ("Lava", ""));

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
