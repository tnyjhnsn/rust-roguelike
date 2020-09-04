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
};
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

pub fn spawn_map(map: &mut Map, ecs: &mut World) {
    map.create_temp_walls();
    for _i in 1..10 {
        let (x, y) = map.get_random_space();
        random_monster(ecs, x, y);
    }
    for _i in 1..15 {
        let (x, y) = map.get_random_space();
        random_item(ecs, x, y);
    }
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 5);
    match roll {
        1 => { white_centipede(ecs, x, y) }
        2 => { red_ant(ecs, x, y) }
        3 => { ghost(ecs, x, y) }
        _ => { grey_mould(ecs, x, y) }
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
        .with(CombatStats{ defense: 1, power: 4 })
        .with(HealthStats{ max_hp: 16, hp: 16 })
        .build();
}

pub fn random_item(ecs: &mut World, x: i32, y: i32) {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1, 6);
    match roll {
        1 => { health_potion(ecs, x, y) }
        2 => { magic_missile_scroll(ecs, x, y) }
        3 => { acid_rain_potion(ecs, x, y) }
        4 => { dragon_breath_potion(ecs, x, y) }
        _ => { confusion_scroll(ecs, x, y) }
    }
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2000 })
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

pub fn dragon_breath_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2101 })
        .with(Position{ x, y })
        .with(Item{})
        .with(Consumeable{})
        .with(Ranged{ range: 6 })
        .with(InflictsDamage{ damage: 10 })
        .with(AreaOfEffect{ radius: 2 })
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

pub fn confusion_scroll(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2103 })
        .with(Position{ x, y })
        .with(Item{})
        .with(Consumeable{})
        .with(Ranged{ range: 6 })
        .with(Confusion{ turns: 3 })
        .with(HealthStats{ max_hp: 1, hp: 1 })
        .build();
}

