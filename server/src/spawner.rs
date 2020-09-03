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
    InflictsDamage,
    ProvidesHealing,
    BlocksTile};
use rand::Rng;

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
        .with(CombatStats{ defense: 2, power: 5 })
        .with(HealthStats{ max_hp: 300, hp: 300 })
        .build()
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let code;
    let roll = rng.gen_range(1, 5);
    match roll {
        1 => { code = 10 }
        2 => { code = 11 }
        3 => { code = 12 }
        _ => { code = 13 }
    }
    ecs
        .create_entity()
        .with(Monster{})
        .with(Code { code })
        .with(Position { x, y })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
        })
        .with(BlocksTile{})
        .with(CombatStats{ defense: 1, power: 4 })
        .with(HealthStats{ max_hp: 16, hp: 16 })
        .build();
}

pub fn random_potion(ecs: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let code;
    let roll = rng.gen_range(1, 5);
    match roll {
        1 => { code = 2000 }
        2 => { code = 2001 }
        3 => { code = 2002 }
        _ => { code = 2101 }
    }
    ecs.create_entity()
        .with(Code { code })
        .with(Position{ x, y })
        .with(Item{})
        .with(Consumeable{})
        .with(ProvidesHealing{ heal: 8 })
        .with(HealthStats{ max_hp: 1, hp: 1 })
        .build();
}

pub fn magic_missile_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2100 })
        .with(Position{ x, y })
        .with(Item{})
        .with(Consumeable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(HealthStats{ max_hp: 1, hp: 1 })
        .build();
}

pub fn acid_rain_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2102 })
        .with(Position{ x, y })
        .with(Item{})
        .with(Consumeable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 8 })
        .with(AreaOfEffect{ radius: 3 })
        .with(HealthStats{ max_hp: 1, hp: 1 })
        .build();
}

pub fn random_item(ecs: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 4);
    match roll {
        1 => { random_potion(ecs, x, y) }
        2 => { magic_missile_scroll(ecs, x, y) }
        _ => { acid_rain_potion(ecs, x, y) }
    }
}

