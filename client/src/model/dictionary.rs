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
        dict.insert(MOB_BLACK_RAT, ("Black Rat", "black-rat"));
        dict.insert(MOB_WOLF, ("Mangy Wolf", "wolf"));
        dict.insert(MOB_KOBOLD, ("Annoying Kobold", "kobold"));
        dict.insert(MOB_THIEF, ("Sneaky Thief", "Thief"));
        dict.insert(ITEM_HEALTH_POTION, ("Health Potion", "health-potion"));
        dict.insert(ITEM_MAGIC_MISSILE, ("Magic Missiles Scroll", "scroll"));
        dict.insert(ITEM_DRAGON_BREATH, ("Dragon Breath", "dragon-breath"));
        dict.insert(ITEM_ACID_RAIN, ("Acid Rain", "acid-rain"));
        dict.insert(ITEM_CONFUSION_SCROLL, ("Scroll of Confusion", "scroll-unlock"));
        dict.insert(LOOT_MEAT, ("Delicious Meat", "trippy-mushroom"));
        dict.insert(LOOT_HIDE, ("Leather Hide", "yellow-mushroom"));
        dict.insert(WEAP_DAGGER, ("Dagger", "dagger"));
        dict.insert(WEAP_RUSTY_SWORD, ("Rusty Old Sword", "long-sword"));
        dict.insert(WEAP_LONG_SWORD, ("Long Sword", "long-sword"));
        dict.insert(ARMOUR_SHIELD, ("Shield", "shield"));
        dict.insert(ARMOUR_TUNIC, ("Stained Tunic", "leather"));
        dict.insert(ARMOUR_PANTS, ("Torn Trousers", "leather"));
        dict.insert(ARMOUR_OLD_BOOTS, ("Old Boots", "leather"));
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
