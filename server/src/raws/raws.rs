use super::*;
use serde::{Deserialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Raws {
    pub entities: Vec<RawEntity>,
    pub spawn_table: Vec<SpawnTableEntry>,
}

impl Raws {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            spawn_table: Vec::new(),
        }
    }
    pub fn load_entities(&mut self, mut entities: Vec<RawEntity>) {
        self.entities.append(&mut entities);
    }
    pub fn load_spawn_table(&mut self, mut spawn_table: Vec<SpawnTableEntry>) {
        self.spawn_table.append(&mut spawn_table);
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

#[derive(Clone, Debug, Deserialize)]
pub struct SpawnTableEntry {
    pub code: i32,
    pub weight: i32,
    pub min_difficulty: i32,
    pub max_difficulty: i32,
    pub add_diff_to_weight: Option<bool>,
}

