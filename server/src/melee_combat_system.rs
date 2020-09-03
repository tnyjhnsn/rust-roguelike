use specs::prelude::*;
use super::{
    CombatStats,
    HealthStats,
    WantsToMelee,
    Code,
    SufferDamage,
    GameLog
};
use roguelike_common::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, Code>,
                        ReadStorage<'a, CombatStats>,
                        ReadStorage<'a, HealthStats>,
                        WriteStorage<'a, SufferDamage>,
                        WriteExpect<'a, GameLog>,
                        );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, codes, combat_stats, health_stats, mut inflict_damage, mut log) = data;
        for (_entity, wants_melee, code, combat, health)
            in (&entities, &wants_melee, &codes, &combat_stats, &health_stats).join() {
            if health.hp > 0 {
                let target_combat = combat_stats.get(wants_melee.target).unwrap();
                let target_health = health_stats.get(wants_melee.target).unwrap();
                if target_health.hp > 0 {
                    let target_code = codes.get(wants_melee.target).unwrap();
                    let damage = i32::max(0, combat.power - target_combat.defense);
                    if damage == 0 {
                        log.add_log(vec![LogType::Melee as i32, code.code, target_code.code, 0]);
                    } else {
                        SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                        log.add_log(vec![LogType::Melee as i32, code.code, target_code.code, damage]);
                    }
                }
            }
        }
        wants_melee.clear();
    }
}
