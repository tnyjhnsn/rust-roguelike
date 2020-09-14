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
    dwarven_mines_gate::*,
    dwarven_mines_hall::*,
};
use rand::Rng;
use std::collections::HashMap;
use roguelike_common::*;

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs
        .create_entity()
        .with(Player{})
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

fn map_table(depth: i32) -> RandomTable {
    RandomTable::new()
        .add(10, 10)
        .add(11, 5)
        .add(12, 5)
        .add(13, 1 + depth)
        .add(2000, 8)
        .add(2100, 5)
        .add(2101, 2 + depth)
        .add(2102, 2 + depth)
        .add(2103, 1 + depth)
        .add(3000, 5)
        .add(3001, 5)
        .add(3002, 5)
        .add(3100, 5)
}

const MAX_MONSTERS : i32 = 10;

pub fn spawn_map(map: &mut Map, ecs: &mut World) {
    map.populate_tiles();
    let spawn_table = map_table(map.depth);
    let mut spawn_points = HashMap::new();

    {
        let mut rng = rand::thread_rng();
        let num_spawns = rng.gen_range(5, MAX_MONSTERS + 1) + (map.depth as f64 * 1.5).floor() as i32;
        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let idx = map.get_random_space();
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
            Some(10) => white_centipede(ecs, x, y),
            Some(11) => red_ant(ecs, x, y),
            Some(12) => ghost(ecs, x, y),
            Some(13) => grey_mould(ecs, x, y),
            Some(2000) => health_potion(ecs, x, y),
            Some(2100) => magic_missile_scroll(ecs, x, y),
            Some(2101) => dragon_breath_potion(ecs, x, y),
            Some(2102) => acid_rain_potion(ecs, x, y),
            Some(2103) => confusion_scroll(ecs, x, y),
            Some(3000) => dagger(ecs, x, y),
            Some(3001) => short_sword(ecs, x, y),
            Some(3002) => long_sword(ecs, x, y),
            Some(3100) => shield(ecs, x, y),
            _ => {},
        }
    }

    for (idx, i) in map.tiles.iter().enumerate() {
        match i {
            TileType::Chasm => {
                let (x, y) = map.idx_xy(idx as i32);
                map.contents[idx].push(chasm_trap(ecs, x, y));
            }
            _ => {},
        }
    }
}

fn white_centipede(ecs: &mut World, x: i32, y: i32) { monster(ecs, 10, x, y); }
fn red_ant(ecs: &mut World, x: i32, y: i32) { monster(ecs, 11, x, y); }
fn ghost(ecs: &mut World, x: i32, y: i32) { monster(ecs, 12, x, y); }
fn grey_mould(ecs: &mut World, x: i32, y: i32) { monster(ecs, 13, x, y); }

pub fn monster(ecs: &mut World, code: i32, x: i32, y: i32) {
    ecs
        .create_entity()
        .with(Monster{})
        .with(Code { code })
        .with(Position { x, y })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .with(BlocksTile{})
        .with(CombatStats { defense: 1, power: 4 })
        .with(HealthStats { max_hp: 16, hp: 16 })
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2000 })
        .with(Position{ x, y })
        .with(Item {})
        .with(Consumeable {})
        .with(ProvidesHealing { heal: 8 })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .build();
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2100 })
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
        .with(Code { code: 2101 })
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
        .with(Code { code: 2102 })
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
        .with(Code { code: 2103 })
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
        .with(Code { code: 3000 })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 2 })
        .build();
}

pub fn short_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 3001 })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 4 })
        .build();
}

pub fn long_sword(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 3002 })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Melee })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(MeleePowerBonus { power: 6 })
        .build();
}

pub fn shield(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 3100 })
        .with(Position { x, y })
        .with(Item {})
        .with(Equippable { slot: ArmourSlot::Shield })
        .with(HealthStats { max_hp: 1, hp: 1 })
        .with(DefenseBonus { defense: 1 })
        .build();
}

pub fn chasm_trap(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Code { code: 5000 })
        .with(Position { x, y })
        .with(InflictsDamage { damage: 1000 })
        .build()
}

