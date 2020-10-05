use std::fs::File;
use super::{Raws, RawEntity};
use roguelike_common::*;

use ron::de::from_reader;

pub fn load_raws() {
    let paths = vec![
        "raws/mobs.ron",
        "raws/items.ron",
        "raws/weapons.ron",
        "raws/traps.ron",
    ];

    let mut master_raws = Raws::new();

    for path in &paths {
        let file = File::open(path).expect("Cannot open file");
        let raws: Raws = from_reader(file).expect("Cannot read from file");
        master_raws.concat(raws);
    }

    // testing
    let ent: Option<RawEntity> = master_raws.entities.into_iter().find(|e| e.code.code == ITEM_HEALTH_POTION);
    if let Some(e) = ent {
        println!("WC {:?}", e);
    }
}
