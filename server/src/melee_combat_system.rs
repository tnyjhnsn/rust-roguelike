use specs::prelude::*;
use super::{
    WantsToMelee,
    Code,
    SufferDamage,
    GameLog,
    Attributes,
    Skills,
    Skill,
    Pools,
    Equipped,
    MeleeWeapon,
    NaturalAttackDefense,
    Wearable,
    WeaponAttribute,
    EquipmentSlot,
    HitDesc,
    skill_bonus,
};
use roguelike_common::*;

pub struct MeleeCombatSystem {}

const BASE_AC: i32 = 10;

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Code>,
        ReadStorage<'a, Attributes>,
        ReadStorage<'a, Skills>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, Pools>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Equipped>,
        ReadStorage<'a, MeleeWeapon>,
        ReadStorage<'a, NaturalAttackDefense>,
        ReadStorage<'a, Wearable>,
        WriteExpect<'a, RandomNumberGenerator>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, codes, attributes, skills,
             mut inflict_damage, pools, mut log, equipped_items,
             melee_weapons, natural, wearables, mut rng) = data;

        for (entity, wants_melee, code, attacker_attributes, attacker_skills, attacker_pools)
            in (&entities, &wants_melee, &codes, &attributes, &skills, &pools).join() {

            let target_pools = pools.get(wants_melee.target).unwrap();
            let target_attributes = attributes.get(wants_melee.target).unwrap();
            let target_skills = skills.get(wants_melee.target).unwrap();

            if attacker_pools.hp.current > 0 && target_pools.hp.current > 0 {
                let target_code = codes.get(wants_melee.target).unwrap();

                let mut weapon_info = MeleeWeapon {
                    range: 0,
                    hit_desc: HitDesc::Attack,
                    attribute: WeaponAttribute::Might,
                    damage_dice: (1, 4, 0),
                    hit_bonus: 0,
                };

                if let Some(natural) = natural.get(entity) {
                    if !natural.attacks.is_empty() {
                        let idx = if natural.attacks.len() == 1 { 0 } else {
                            rng.roll_dice(1, natural.attacks.len() as i32, -1) as usize 
                        };
                        let attacks = natural.attacks[idx];
                        weapon_info.attribute = attacks.attribute;
                        weapon_info.damage_dice = attacks.damage_dice;
                        weapon_info.hit_bonus = attacks.hit_bonus;
                    }
                }

                for (wielded, melee) in (&equipped_items, &melee_weapons).join() {
                    if wielded.owner == entity && wielded.slot == EquipmentSlot::Melee {
                        weapon_info = melee.clone();
                    }
                }

                let natural_roll = rng.roll_dice(1, 20, 0);
                let attr_hit_bonus = if weapon_info.attribute == WeaponAttribute::Might {
                    attacker_attributes.might.bonus
                } else {
                    attacker_attributes.quickness.bonus
                };
                let skill_hit_bonus = skill_bonus(Skill::Melee, &*attacker_skills);
                let weapon_hit_bonus = weapon_info.hit_bonus;
                let modified_hit_roll = natural_roll + attr_hit_bonus
                    + skill_hit_bonus + weapon_hit_bonus;

                let mut armour_item_bonus_f = 0.0;
                for (wielded,armour) in (&equipped_items, &wearables).join() {
                    if wielded.owner == wants_melee.target {
                        armour_item_bonus_f += armour.armour_class;
                    }
                }
                let base_armour_class = match natural.get(wants_melee.target) {
                    None => BASE_AC,
                    Some(natural) => natural.armour_class.unwrap_or(BASE_AC),
                };
                let armour_quickness_bonus = target_attributes.quickness.bonus;
                let armour_skill_bonus = skill_bonus(Skill::Defense, &*target_skills);
                let armour_item_bonus = armour_item_bonus_f as i32;
                let armour_class = base_armour_class + armour_quickness_bonus + armour_skill_bonus
                    + armour_item_bonus;

                if natural_roll != 1 && (natural_roll == 20 || modified_hit_roll > armour_class) {
                    let (n, d, b) = weapon_info.damage_dice;
                    let base_damage = rng.roll_dice(n, d, b);
                    let attr_damage_bonus = attacker_attributes.might.bonus;
                    let skill_damage_bonus = skill_bonus(Skill::Melee, &*attacker_skills);

                    let damage = i32::max(0, base_damage + attr_damage_bonus + skill_hit_bonus
                        + skill_damage_bonus);
                    SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, damage);
                    log.add_log(vec![LogType::Melee as i32, code.code, target_code.code, damage]);
                } else if natural_roll == 1 {
                    log.add_log(vec![LogType::NaturalMiss as i32, code.code, target_code.code]);
                } else {
                    log.add_log(vec![LogType::Miss as i32, code.code, target_code.code]);
                }
            
            }
        }
        wants_melee.clear();
    }
}
