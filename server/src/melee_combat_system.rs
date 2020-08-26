use specs::prelude::*;
use super::{CombatStats, WantsToMelee, Code, SufferDamage, GameLog};
use roguelike_common::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, Code>,
                        ReadStorage<'a, CombatStats>,
                        WriteStorage<'a, SufferDamage>,
                        WriteExpect<'a, GameLog>,
                        );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, codes, combat_stats, mut inflict_damage, mut log) = data;
        for (_entity, wants_melee, code, stats)
            in (&entities, &wants_melee, &codes, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_code = codes.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, stats.power - target_stats.defense);
                    if damage == 0 {
                        log.add_log(vec![LogType::Melee as i32, code.code, target_code.code, 0]);
                    } else {
                        log.add_log(vec![LogType::Melee as i32, code.code, target_code.code, damage]);
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);

                    }
                }
            }
        }
        wants_melee.clear();
    }
}
