use specs::prelude::*;
use super::{
    CombatStats,
    HealthStats,
    Player,
    Code,
    Position,
    FieldOfView,
    Map,
    raws::*,
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

pub fn spawn_map(map: &mut Map, ecs: &mut World) {
    let spawn_table = get_spawn_table(&RAWS.lock().unwrap(), map.difficulty);
    let mut spawn_points = HashMap::new();

    {
        let mut rng = ecs.fetch_mut::<RandomNumberGenerator>();
        let num_spawns = rng.roll_dice(5, 6, 0) + (map.difficulty as f64 * 1.5).floor() as i32;
        for _i in 0..num_spawns {
            let mut added = false;
            while !added {
                let idx = map.get_random_space(&mut rng);
                if !spawn_points.contains_key(&idx) {
                    spawn_points.insert(idx, spawn_table.roll(&mut rng));
                    added = true;
                }
            }
        }
    }

    for spawn in &spawn_points {
        let pos = *spawn.0;
        if spawn.1.is_some() {
            spawn_from_raws(&RAWS.lock().unwrap(), ecs.create_entity(), &spawn.1.unwrap(), pos);
        }
    }

    for (idx, i) in map.tiles.iter().enumerate() {
        let pos = map.idx_xy(idx as i32);
        match i {
            TileType::Chasm => spawn_from_raws(&RAWS.lock().unwrap(),
                ecs.create_entity(), &TRAP_CHASM, pos),
            TileType::Lava => spawn_from_raws(&RAWS.lock().unwrap(),
                ecs.create_entity(), &TRAP_LAVA, pos),
            _ => {},
        }
    }
}

