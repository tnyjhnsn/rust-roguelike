use specs::prelude::*;
use super::{
    SufferDamage,
    Player,
    GameLog,
    Code,
    RunState,
    Pools,
    Equipped,
    InInventory,
    Position,
    LootTable,
    raws::*,
};
use roguelike_common::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, Pools>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp.current -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs : &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<Pools>();
        let players = ecs.read_storage::<Player>();
        let codes = ecs.read_storage::<Code>();
        let entities = ecs.entities();
        let mut log = ecs.fetch_mut::<GameLog>();
        let mut state = ecs.fetch_mut::<RunState>();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp.current < 1 { 
                let player = players.get(entity);
                match player {
                    None => {
                        let victim = codes.get(entity);
                        if let Some(victim) = victim {
                            if victim.code < 2000 {
                                log.add_log(vec![LogType::Dead as i32, victim.code]);
                            } else {
                                // TODO items need to have hp for this to work
                                log.add_log(vec![LogType::Destroyed as i32, victim.code]);
                            }
                            dead.push(entity);
                            state.add_state(CONTENTS_CHANGE);
                        }
                    },
                    Some(_) =>  {
                        state.add_state(GAME_OVER);
                        state.add_state(INVENTORY_CHANGE);
                        state.add_state(ARMOUR_CHANGE);
                    }
                }
            }
        }
    }

    let mut to_spawn = Vec::new();
    {
        let mut to_drop = Vec::new();
        let entities = ecs.entities();
        let mut equipped = ecs.write_storage::<Equipped>();
        let mut carried = ecs.write_storage::<InInventory>();
        let mut positions = ecs.write_storage::<Position>();
        let loot_table = ecs.read_storage::<LootTable>();
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        for victim in &dead {
            let pos = positions.get(*victim);
            for (entity, equipped) in (&entities, &equipped).join() {
                if equipped.owner == *victim {
                    if let Some(pos) = pos {
                        to_drop.push((entity, pos.clone()));
                    }
                }
            }
            for (entity, inventory) in (&entities, &carried).join() {
                if inventory.owner == *victim {
                    if let Some(pos) = pos {
                        to_drop.push((entity, pos.clone()));
                    }
                }
            }
            if let Some(table) = loot_table.get(*victim) {
                let drop_finder = get_item_drop(
                    &RAWS.lock().unwrap(),
                    &mut rng,
                    table.key
                );
                if let Some(drop) = drop_finder {
                    if let Some(pos) = pos {
                        to_spawn.push((drop, pos.clone()));
                    }
                }
            }
        }
        for drop in &to_drop {
            equipped.remove(drop.0);
            carried.remove(drop.0);
            positions.insert(drop.0, drop.1.clone()).expect("Unable to insert Position");
        }
    }

    {
        for drop in &to_spawn {
            let (x, y) = (drop.1.x, drop.1.y);
            spawn_from_raws(&RAWS.lock().unwrap(), ecs, &drop.0,
                SpawnType::AtPosition{ x, y });
        }
    }

    for victim in &dead {
        ecs.delete_entity(*victim).expect("Unable to delete");
    }    
}
