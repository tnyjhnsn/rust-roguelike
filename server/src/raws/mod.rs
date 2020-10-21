use std::fs::File;
use std::sync::Mutex;
use ron::de::from_reader;
use std::collections::HashMap;

use super::*;

pub mod raws;
pub use raws::*;

lazy_static! {
    pub static ref RAWS: Mutex<Raws> = Mutex::new(Raws::new());
}

pub fn load_raws() {
    let paths = vec![
        "raws/mobs.ron",
        "raws/items.ron",
        "raws/weapons_armour.ron",
        "raws/traps.ron",
    ];

    for path in &paths {
        let file = File::open(path).expect("Cannot open file");
        let raws: Vec<RawEntity> = from_reader(file).expect("Cannot read from file");
        &RAWS.lock().unwrap().load_entities(raws);
    }

    let mut file = File::open("raws/spawn_table.ron").expect("Cannot open file");
    let spawns: Vec<SpawnTableEntry> = from_reader(file).expect("Cannot read from file");
    &RAWS.lock().unwrap().load_spawn_table(spawns);

    file = File::open("raws/loot_table.ron").expect("Cannot open file");
    let loots: HashMap<LootTableKey, Vec<LootDrop>> = from_reader(file).expect("Cannot read from file");
    &RAWS.lock().unwrap().load_loot_table(loots);

    // testing
    //println!("{:?}", &RAWS.lock().unwrap().entities);
}
