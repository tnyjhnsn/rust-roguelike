use specs::prelude::*;
use super::{
    CombatStats,
    Player,
    Code,
    Position,
    FieldOfView,
    Monster,
    Item,
    Potion,
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
        .with(CombatStats{ max_hp: 30, hp: 30, defense: 2, power: 5 })
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
        .with(CombatStats{ max_hp: 16, hp: 16, defense: 1, power: 4 })
        .build();
}

pub fn health_potion(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Code { code: 2000 })
        .with(Position{ x, y })
        .with(Item{})
        .with(Potion{ heal: 8 })
        .build();
}
