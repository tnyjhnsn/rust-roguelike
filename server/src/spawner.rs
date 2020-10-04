use specs::prelude::*;
use super::{
    CombatStats,
    HealthStats,
    Player,
    Code,
    Position,
    FieldOfView,
    Monster,
    Item,
    Consumeable,
    Ranged,
    AreaOfEffect,
    Confusion,
    InflictsDamage,
    ProvidesHealing,
    BlocksTile,
    Map,
    RandomTable,
    Equippable,
    ArmourSlot,
    MeleePowerBonus,
    DefenseBonus,
    EntryTrigger,
};
use std::collections::HashMap;
use roguelike_common::*;

pub fn player(ecs: &mut World, x: i32, y: i32) -> PlayerEntity {
    ecs.create_entity()
        .with(Player {})
        .with(Code { code: 0 })
        .with(Position { x, y })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
        })
        .with(CombatStats { defense: 2, power: 5 })
        .with(HealthStats { max_hp: 300, hp: 300 })
        .build()
}

fn map_table(difficulty: i32) -> RandomTable {
    RandomTable::new()
        .add(MOB_WHITE_CENTIPEDE, 10)
        .add(MOB_RED_ANT, 5)
        .add(MOB_GHOST, 5)
        .add(MOB_GREY_MOULD, 1 + difficulty)
        .add(ITEM_HEALTH_POTION, 8)
        .add(ITEM_MAGIC_MISSILE, 5)
        .add(ITEM_DRAGON_BREATH, 2 + difficulty)
        .add(ITEM_ACID_RAIN, 2 + difficulty)
        .add(ITEM_CONFUSION_SCROLL, 1 + difficulty)
        .add(WEAP_DAGGER, 5)
        .add(WEAP_SHORT_SWORD, 5)
        .add(WEAP_LONG_SWORD, 5)
        .add(WEAP_SHIELD, 5)
}

pub fn spawn_map(map: &mut Map, ecs: &mut World) {
    let spawn_table = map_table(map.difficulty);
    let mut spawn_points = HashMap::new();

    {
        let mut rng = ecs.fetch_mut::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(5, 6, 0) + (map.difficulty as f64 * 1.5).floor() as i32;
        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let idx = map.get_random_space(&mut rng);
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll());
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    for spawn in spawn_points.iter() {
        let (x, y) = *spawn.0;
        match spawn.1 {
            Some(MOB_WHITE_CENTIPEDE) => white_centipede(ecs, x, y),
            Some(MOB_RED_ANT) => red_ant(ecs, x, y),
            Some(MOB_GHOST) => ghost(ecs, x, y),
            Some(MOB_GREY_MOULD) => grey_mould(ecs, x, y),
            Some(ITEM_HEALTH_POTION) => health_potion(ecs, x, y),
            Some(ITEM_MAGIC_MISSILE) => magic_missile_scroll(ecs, x, y),
            Some(ITEM_DRAGON_BREATH) => dragon_breath_potion(ecs, x, y),
            Some(ITEM_ACID_RAIN) => acid_rain_potion(ecs, x, y),
            Some(ITEM_CONFUSION_SCROLL) => confusion_scroll(ecs, x, y),
            Some(WEAP_DAGGER) => dagger(ecs, x, y),
            Some(WEAP_SHORT_SWORD) => short_sword(ecs, x, y),
            Some(WEAP_LONG_SWORD) => long_sword(ecs, x, y),
            Some(WEAP_SHIELD) => shield(ecs, x, y),
            _ => {},
        }
    }

    for (idx, i) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_xy(idx as i32);
        match i {
            TileType::Chasm => chasm_trap(ecs, x, y),
            TileType::Lava => lava_trap(ecs, x, y), 
            _ => {},
        }
    }
}

fn white_centipede(ecs: &mut World, x: i32, y: i32) { monster(ecs, MOB_WHITE_CENTIPEDE, x, y); }
fn red_ant(ecs: &mut World, x: i32, y: i32) { monster(ecs, MOB_RED_ANT, x, y); }
fn ghost(ecs: &mut World, x: i32, y: i32) { monster(ecs, MOB_GHOST, x, y); }
fn grey_mould(ecs: &mut World, x: i32, y: i32) { monster(ecs, MOB_GREY_MOULD, x, y); }

pub fn monster(ecs: &mut World, code: i32, x: i32, y: i32) {
    ecs.create_entity()
        .with(Monster {})
        .with(Code { code })
        .with(Position { x, y })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .with(BlocksTile {})
        .with(CombatStats { defense: 1, power: 4 })
        .with(HealthStats { max_hp: 16, hp: 16 })
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: ITEM_HEALTH_POTION })
        .with(Position { x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(ProvidesHealing { heal: 8 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: ITEM_MAGIC_MISSILE })
        .with(Position { x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn dragon_breath_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: ITEM_DRAGON_BREATH })
        .with(Position { x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 10 })
        .with(AreaOfEffect { radius: 2 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn acid_rain_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: ITEM_ACID_RAIN })
        .with(Position { x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(Ranged { range: 6 })
        .with(InflictsDamage { damage: 8 })
        .with(AreaOfEffect { radius: 3 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: ITEM_CONFUSION_SCROLL })
        .with(Position { x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(Ranged { range: 6 })
        .with(Confusion { turns: 3 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn dagger(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: WEAP_DAGGER })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 2 })
        .build();
}

pub fn short_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: WEAP_SHORT_SWORD })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 4 })
        .build();
}

pub fn long_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: WEAP_LONG_SWORD })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 6 })
        .build();
}

pub fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: WEAP_SHIELD })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Shield })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(DefenseBonus { defense: 1 })
        .build();
}

pub fn chasm_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: TRAP_CHASM })
        .with(Position { x, y })
        .with(EntryTrigger {})
        .with(InflictsDamage { damage: 1000 })
        .build();
}

pub fn lava_trap(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: TRAP_LAVA })
        .with(Position { x, y })
        .with(EntryTrigger {})
        .with(InflictsDamage { damage: 1000 })
        .build();
}

