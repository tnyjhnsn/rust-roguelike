use specs::prelude::*;
use super::{
    Player,
    Code,
    Position,
    FieldOfView,
    Map,
    Attributes,
    Attribute,
    Skills,
    Skill,
    Pools,
    Pool,
    Initiative,
    Faction,
    FactionName,
    raws::*,
};
use std::collections::HashMap;
use roguelike_common::*;
use crate::{attr_bonus, player_hp_at_level, mana_at_level};

const ATTR_BASE: i32 = 11;

pub fn player(ecs: &mut World, x: i32, y: i32) -> PlayerEntity {
    let mut skills = Skills { skills: HashMap::new() };
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Magic, 1);
    let hp = Pool {
        current: player_hp_at_level(ATTR_BASE, 1),
        max: player_hp_at_level(ATTR_BASE, 1),
    };
    let mana = Pool {
        current: mana_at_level(ATTR_BASE, 1),
        max: mana_at_level(ATTR_BASE, 1),
    };
    let player = ecs.create_entity()
        .with(Player {})
        .with(Code { code: 0 })
        .with(Position { x, y })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .with(Attributes {
            might: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
            fitness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
            quickness: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
            intelligence: Attribute { base: ATTR_BASE, modifiers: 0, bonus: attr_bonus(ATTR_BASE) },
        })
        .with(skills)
        .with(Pools {
            hp,
            mana,
            xp: 0,
            level: 1,
        })
        .with(Initiative { current: 0 })
        .with(Faction { name: FactionName::Player })
        .build();

        spawn_from_raws(&RAWS.lock().unwrap(), ecs, &3001,
            SpawnType::Equipped{ owner: player });
        spawn_from_raws(&RAWS.lock().unwrap(), ecs, &3300,
            SpawnType::Equipped{ owner: player });
        spawn_from_raws(&RAWS.lock().unwrap(), ecs, &3400,
            SpawnType::Equipped{ owner: player });
        spawn_from_raws(&RAWS.lock().unwrap(), ecs, &3500,
            SpawnType::Equipped{ owner: player });

        player
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
            spawn_from_raws(&RAWS.lock().unwrap(), ecs,
                &spawn.1.unwrap(), SpawnType::AtPosition{ x: pos.x, y: pos.y });
        }
    }

    for (idx, i) in map.tiles.iter().enumerate() {
        let pos = map.idx_xy(idx as i32);
        let (x, y) = (pos.x, pos.y);
        match i {
            TileType::Door => spawn_from_raws(&RAWS.lock().unwrap(),
                ecs, &OTHER_DOOR, SpawnType::AtPosition{ x, y }),
            TileType::Chasm => spawn_from_raws(&RAWS.lock().unwrap(),
                ecs, &TRAP_CHASM, SpawnType::AtPosition{ x, y }),
            TileType::Lava => spawn_from_raws(&RAWS.lock().unwrap(),
                ecs, &TRAP_LAVA, SpawnType::AtPosition{ x, y }),
            _ => {},
        }
    }
}

