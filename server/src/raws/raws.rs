use std::fs::File;
use ron::de::from_reader;

use super::*;
use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Raws {
    pub entities: Vec<RawEntity>,
}

impl Raws {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn concat(&mut self, mut other: Raws) {
        self.entities.append(&mut other.entities);
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawEntity {
    pub code: Code,
    pub item: Option<Item>,
    pub monster: Option<Monster>,
    pub blocks_tile: Option<BlocksTile>,
    pub consumeable: Option<Consumeable>,
    pub provides_healing: Option<ProvidesHealing>,
    pub ranged: Option<Ranged>,
    pub inflicts_damage: Option<InflictsDamage>,
    pub area_of_effect: Option<AreaOfEffect>,
    pub confusion: Option<Confusion>,
    pub health_stats: Option<HealthStats>,
    pub equipment: Option<Equippable>,
    pub melee_power_bonus: Option<MeleePowerBonus>,
    pub defense_bonus: Option<DefenseBonus>,
    pub field_of_view: Option<FieldOfView>,
    pub entry_trigger: Option<EntryTrigger>,
}

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
        println!("{:?}", e);
    }
}
