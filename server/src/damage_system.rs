use specs::prelude::*;
use super::{
    HealthStats,
    SufferDamage,
    Player,
    GameLog,
    Code,
    RunState,
};
use roguelike_common::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, HealthStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs : &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let health_stats = ecs.read_storage::<HealthStats>();
        let players = ecs.read_storage::<Player>();
        let codes = ecs.read_storage::<Code>();
        let entities = ecs.entities();
        let mut log = ecs.fetch_mut::<GameLog>();
        let mut state = ecs.fetch_mut::<RunState>();
        for (entity, stats) in (&entities, &health_stats).join() {
            if stats.hp < 1 { 
                dead.push(entity);
                let player = players.get(entity);
                match player {
                    None => {},
                    Some(_) =>  {
                        state.add_state(GAME_OVER);
                        state.add_state(INVENTORY_CHANGE);
                        state.add_state(ARMOUR_CHANGE);
                    }
                }
                let victim = codes.get(entity);
                if let Some(victim) = victim {
                    if victim.code < 2000 {
                        log.add_log(vec![LogType::Dead as i32, victim.code]);
                    } else {
                        log.add_log(vec![LogType::Destroyed as i32, victim.code]);
                    }
                    state.add_state(CONTENTS_CHANGE);
                }
            }
        }
    }

    for victim in &dead {
        ecs.delete_entity(*victim).expect("Unable to delete");
    }    
}
