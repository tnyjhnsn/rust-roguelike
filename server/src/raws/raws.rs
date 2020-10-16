use super::*;
use serde::{Deserialize};
use crate::attr_bonus;
use std::collections::HashMap;

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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MobAttributes {
    pub might: Option<i32>,
    pub fitness: Option<i32>,
    pub quickness: Option<i32>,
    pub intelligence: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawEntity {
    pub code: Code,
    pub item: Option<Item>,
    pub monster: Option<Monster>,
    pub bystander: Option<Bystander>,
    pub vendor: Option<Vendor>,
    pub attributes: Option<MobAttributes>,
    pub skills: Option<HashMap<Skill, i32>>,
    pub blocks_tile: Option<BlocksTile>,
    pub consumeable: Option<Consumeable>,
    pub provides_healing: Option<ProvidesHealing>,
    pub ranged: Option<Ranged>,
    pub inflicts_damage: Option<InflictsDamage>,
    pub area_of_effect: Option<AreaOfEffect>,
    pub confusion: Option<Confusion>,
    pub combat_stats: Option<CombatStats>,
    pub health_stats: Option<HealthStats>,
    pub equippable: Option<Equippable>,
    pub melee_power_bonus: Option<MeleePowerBonus>,
    pub defense_bonus: Option<DefenseBonus>,
    pub field_of_view: Option<i32>,
    pub entry_trigger: Option<EntryTrigger>,
    pub door: Option<Door>,
    pub blocks_visibility: Option<BlocksVisibility>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpawnTableEntry {
    pub code: i32,
    pub weight: i32,
    pub min_difficulty: i32,
    pub max_difficulty: i32,
    pub add_diff_to_weight: Option<bool>,
}

const ATTR_BASE: i32 = 11;

pub fn spawn_from_raws(raws: &Raws, new_entity: EntityBuilder, code: &i32,
    pos: Position) {

    let mut entity = new_entity;

    let template = &raws.entities.iter().find(|e| e.code.code == *code);
    if let Some(t) = template {
        entity = entity.with(t.code);
        entity = entity.with(pos);
        if let Some(item) = t.item { entity = entity.with(item); }
        if let Some(monster) = t.monster { entity = entity.with(monster); }
        if let Some(bystander) = t.bystander { entity = entity.with(bystander); }
        if let Some(vendor) = t.vendor { entity = entity.with(vendor); }
        if let Some(blocks_tile) = t.blocks_tile { entity = entity.with(blocks_tile); }
        if let Some(consumeable) = t.consumeable { entity = entity.with(consumeable); }
        if let Some(provides_healing) = t.provides_healing { entity = entity.with(provides_healing); }
        if let Some(ranged) = t.ranged { entity = entity.with(ranged); }
        if let Some(inflicts_damage) = t.inflicts_damage { entity = entity.with(inflicts_damage); }
        if let Some(area_of_effect) = t.area_of_effect { entity = entity.with(area_of_effect); }
        if let Some(confusion) = t.confusion { entity = entity.with(confusion); }
        if let Some(combat_stats) = t.combat_stats { entity = entity.with(combat_stats); }
        if let Some(health_stats) = t.health_stats { entity = entity.with(health_stats); }
        if let Some(equippable) = t.equippable { entity = entity.with(equippable); }
        if let Some(melee_power_bonus) = t.melee_power_bonus { entity = entity.with(melee_power_bonus); }
        if let Some(defense_bonus) = t.defense_bonus { entity = entity.with(defense_bonus); }
        if let Some(entry_trigger) = t.entry_trigger { entity = entity.with(entry_trigger); }
        if let Some(door) = t.door { entity = entity.with(door); }
        if let Some(blocks_visibility) = t.blocks_visibility { entity = entity.with(blocks_visibility); }
        if let Some(field_of_view) = t.field_of_view {
            entity = entity.with(FieldOfView { visible_tiles: Vec::new(), range: field_of_view });
        }
        if let Some(s) = &t.skills {
            let mut skills = Skills { skills: HashMap::new() };
            skills.skills.insert(Skill::Melee, 1);
            skills.skills.insert(Skill::Defense, 1);
            skills.skills.insert(Skill::Magic, 1);
            for (k, v) in s {
                skills.skills.insert(*k, *v);
            }
            entity = entity.with(skills);
        }
        if let Some(attributes) = t.attributes {
            let mut attr = Attributes {
                might: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                fitness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                quickness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                intelligence: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
            };
            if let Some(might) = attributes.might {
                attr.might = Attribute { base: might, modifiers: 0, bonus: attr_bonus(might) };
            }
            if let Some(fitness) = attributes.fitness {
                attr.fitness = Attribute { base: fitness, modifiers: 0, bonus: attr_bonus(fitness) };
            }
            if let Some(quickness) = attributes.quickness {
                attr.quickness = Attribute { base: quickness, modifiers: 0, bonus: attr_bonus(quickness) };
            }
            if let Some(intelligence) = attributes.intelligence {
                attr.intelligence = Attribute { base: intelligence, modifiers: 0, bonus: attr_bonus(intelligence) };
            }
            entity = entity.with(attr);
        }
        entity.build();
    }
}

pub fn get_spawn_table(raws: &Raws, difficulty: i32) -> RandomTable {
    let options: Vec<&SpawnTableEntry> = raws.spawn_table
        .iter()
        .filter(|o| difficulty >= o.min_difficulty && difficulty <= o.max_difficulty)
        .collect();
    let mut table = RandomTable::new();
    for o in options.iter() {
        let mut weight = o.weight;
        if o.add_diff_to_weight.is_some() {
            weight += difficulty;
        }
        table = table.add(o.code, weight);
    }
    table
}
