use super::*;
use serde::{Deserialize};
use crate::attr_bonus;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpawnType {
    AtPosition { x: i32, y: i32 },
    Equipped { owner: Entity, slot: EquipmentSlot },
    Carried { owner: Entity },
}

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

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize)]
pub enum MobAttributes {
    Might,
    Fitness,
    Quickness,
    Intelligence,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawEntity {
    pub code: Code,
    pub item: Option<Item>,
    pub monster: Option<Monster>,
    pub bystander: Option<Bystander>,
    pub vendor: Option<Vendor>,
    pub attributes: Option<HashMap<MobAttributes, i32>>,
    pub skills: Option<HashMap<Skill, i32>>,
    pub blocks_tile: Option<BlocksTile>,
    pub consumeable: Option<Consumeable>,
    pub provides_healing: Option<ProvidesHealing>,
    pub ranged: Option<Ranged>,
    pub inflicts_damage: Option<InflictsDamage>,
    pub area_of_effect: Option<AreaOfEffect>,
    pub confusion: Option<Confusion>,
    pub weapon: Option<MeleeWeapon>,
    pub wearable: Option<Wearable>,
    pub field_of_view: Option<i32>,
    pub entry_trigger: Option<EntryTrigger>,
    pub door: Option<Door>,
    pub blocks_visibility: Option<BlocksVisibility>,
    pub level: Option<i32>,
    pub equipped: Option<Vec<(i32, EquipmentSlot)>>,
    pub natural: Option<NaturalAttackDefense>,
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

pub fn spawn_from_raws(raws: &Raws, ecs: &mut World, code: &i32,
    pos: SpawnType) {

    let mut entity = ecs.create_entity();

    match pos {
        SpawnType::AtPosition{ x, y } => entity = entity.with(Position { x, y }),
        SpawnType::Carried{ owner } => entity = entity.with(InInventory { owner }),
        SpawnType::Equipped{ owner, slot } => entity = entity.with(Equipped { owner, slot }),
    };

    let template = &raws.entities.iter().find(|e| e.code.code == *code);
    if let Some(t) = template {
        entity = entity.with(t.code);
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
        if let Some(entry_trigger) = t.entry_trigger { entity = entity.with(entry_trigger); }
        if let Some(door) = t.door { entity = entity.with(door); }
        if let Some(blocks_visibility) = t.blocks_visibility { entity = entity.with(blocks_visibility); }
        if let Some(field_of_view) = t.field_of_view {
            entity = entity.with(FieldOfView { visible_tiles: Vec::new(), range: field_of_view });
        }
        if let Some(weapon) = t.weapon {
            entity = entity.with(Equippable { slot: EquipmentSlot::Melee });
            entity = entity.with(weapon);
        }
        if let Some(wearable) = t.wearable {
            entity = entity.with(Equippable { slot: wearable.slot });
            entity = entity.with(wearable);
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
        if let Some(attributes) = &t.attributes {
            let mut mob_fitness = ATTR_BASE;
            let mut mob_intelligence = ATTR_BASE;
            let mut attr = Attributes {
                might: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                fitness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                quickness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
                intelligence: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
            };
            if attributes.contains_key(&MobAttributes::Might) {
                let value = attributes[&MobAttributes::Might];
                attr.might = Attribute { base: value, modifiers: 0, bonus: attr_bonus(value) };
            }
            if attributes.contains_key(&MobAttributes::Fitness) {
                let value = attributes[&MobAttributes::Fitness];
                attr.fitness = Attribute { base: value, modifiers: 0, bonus: attr_bonus(value) };
                mob_fitness = value;
            }
            if attributes.contains_key(&MobAttributes::Quickness) {
                let value = attributes[&MobAttributes::Quickness];
                attr.quickness = Attribute { base: value, modifiers: 0, bonus: attr_bonus(value) };
                mob_intelligence = value;
            }
            if attributes.contains_key(&MobAttributes::Intelligence) {
                let value = attributes[&MobAttributes::Intelligence];
                attr.intelligence = Attribute { base: value, modifiers: 0, bonus: attr_bonus(value) };
            }
            entity = entity.with(attr);

            let mut mob_level = 1;
            if let Some(level) = t.level { mob_level = level; };
            let mob_hp = npc_hp(mob_fitness, mob_level);
            let mob_mana = mana_at_level(mob_intelligence, mob_level);
            let pools = Pools {
                level: mob_level,
                xp: 0,
                hp: Pool { current: mob_hp, max: mob_hp },
                mana: Pool { current: mob_mana, max: mob_mana },
            };
            entity = entity.with(pools);
        }
        if let Some(natural) = &t.natural {
            let mut nat = NaturalAttackDefense {
                armour_class: natural.armour_class,
                attacks: Vec::new(),
            };
            for a in &natural.attacks {
                nat.attacks.push(*a);
            }
            entity = entity.with(nat);
        }

        let mob = entity.build();

        // TODO Needs testing
        if let Some(equipped) = &t.equipped {
            for (code, slot) in equipped {
                spawn_from_raws(raws, ecs, code, SpawnType::Equipped{ owner: mob, slot: *slot });
            }
        }
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
