use specs::prelude::*;
use super::{
    CombatStats,
    HealthStats,
    WantsToMelee,
    Code,
    SufferDamage,
    MeleePowerBonus,
    DefenseBonus,
    Equipped,
    GameLog
};
use roguelike_common::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Code>,
        ReadStorage<'a, CombatStats>,
        ReadStorage<'a, HealthStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, MeleePowerBonus>,
        ReadStorage<'a, DefenseBonus>,
        ReadStorage<'a, Equipped>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, codes, combat_stats, health_stats,
             mut inflict_damage, melee_bonus, defense_bonus, equipped, mut log) = data;
        for (entity, wants_melee, code, combat, health)
            in (&entities, &wants_melee, &codes, &combat_stats, &health_stats).join() {
            if health.hp > 0 {
                let mut offense_extra = 0;
                for (_item, power_bonus, equipped_by) in (&entities, &melee_bonus, &equipped).join() {
                    if equipped_by.owner == entity {
                        offense_extra += power_bonus.power;
                    }
                }

                let target_combat = combat_stats.get(wants_melee.target).unwrap();
                let target_health = health_stats.get(wants_melee.target).unwrap();
                if target_health.hp > 0 {
                    let target_code = codes.get(wants_melee.target).unwrap();
                    let mut defense_extra = 0;
                    for (_item, defense_bonus, equipped_by) in (&entities, &defense_bonus, &equipped).join() {
                        if equipped_by.owner == wants_melee.target {
                            defense_extra += defense_bonus.defense;
                        }
                    }
                    let damage = i32::max(0, (combat.power + offense_extra) - (target_combat.defense + defense_extra));
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
