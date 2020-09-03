use specs::prelude::*;
use super::{
    WantsToPickupItem,
    WantsToDropItem,
    WantsToUseItem,
    Code,
    Map,
    InInventory,
    Position,
    GameLog,
    Consumeable,
    ProvidesHealing,
    InflictsDamage,
    AreaOfEffect,
    SufferDamage,
    HealthStats,
    RunState,
};
use roguelike_common::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct PickupItemSystem {}

impl<'a> System<'a> for PickupItemSystem {
    type SystemData = (ReadExpect<'a, Entity>,
                       WriteExpect<'a, GameLog>,
                       WriteStorage<'a, WantsToPickupItem>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Code>,
                       WriteStorage<'a, InInventory>,
                       WriteExpect<'a, RunState>,
                       );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, mut wants_pickup, mut positions, codes, mut inventory, mut state) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            inventory.insert(pickup.item,
                InInventory { owner: pickup.collected_by }).expect("Unable to insert Inventory entry");

            if pickup.collected_by == *player {
                let item_code = codes.get(pickup.item).unwrap().code;
                gamelog.add_log(vec![LogType::Collect as i32, 0, item_code]);
                state.add_state(INVENTORY_CHANGE);
            }
        }
        wants_pickup.clear();
    }
}

pub struct DropItemSystem {}

impl<'a> System<'a> for DropItemSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToDropItem>,
                        ReadStorage<'a, Code>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, InInventory>,
                        WriteExpect<'a, RunState>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, entities, mut wants_drop, codes, mut positions, mut inventory, mut state) = data;

        for (entity, to_drop) in (&entities, &wants_drop).join() {
            let mut dropper_pos: Position = Position{ x: 0, y: 0 };
            {
                let dropped_pos = positions.get(entity).unwrap();
                dropper_pos.x = dropped_pos.x;
                dropper_pos.y = dropped_pos.y;
            }
            positions.insert(to_drop.item, Position{ x: dropper_pos.x, y: dropper_pos.y }).expect("Unable to insert position");
            inventory.remove(to_drop.item);

            if entity == *player {
                let item_code = codes.get(to_drop.item).unwrap().code;
                gamelog.add_log(vec![LogType::Drop as i32, 0, item_code]);
                state.add_state(INVENTORY_CHANGE);
            }
        }
        wants_drop.clear();
    }
}

pub struct UseItemSystem {}

impl<'a> System<'a> for UseItemSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        ReadExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, WantsToUseItem>,
                        ReadStorage<'a, Code>,
                        ReadStorage<'a, Consumeable>,
                        ReadStorage<'a, ProvidesHealing>,
                        ReadStorage<'a, InflictsDamage>,
                        WriteStorage<'a, SufferDamage>,
                        WriteStorage<'a, HealthStats>,
                        ReadStorage<'a, AreaOfEffect>,
                        WriteExpect<'a, RunState>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, map, entities, mut wants_use, codes, consumeables,
             healing, inflict_damage, mut suffer_damage, mut health_stats, aoe, mut state) = data;

        for (entity, use_item) in (&entities, &wants_use).join() {
            let mut used_item = true;

            let mut targets = Vec::new();
            match use_item.target {
                Some(_t) => {
                    let area_effect = aoe.get(use_item.item);
                    match area_effect {
                        Some(_a) => {
                            let idx = use_item.target.unwrap();
                            let mut aoe = vec![idx];
                            map.get_area_of_effect(&mut aoe, 3);
                            let aoe_tiles: HashSet<i32> = HashSet::from_iter(aoe);
                            for tile in aoe_tiles.iter() {
                                for mob in map.contents[*tile as usize].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                        None => {
                            let idx = use_item.target.unwrap();
                            for mob in map.contents[idx as usize].iter() {
                                targets.push(*mob);
                            }
                        }
                    }
                }
                None => targets.push(*player),
            }

            let item_heals = healing.get(use_item.item);
            match item_heals {
                Some(item) => {
                    for target in targets.iter() {
                        let stats = health_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + item.heal);
                            if entity == *player {
                                let item_code = codes.get(use_item.item).unwrap().code;
                                gamelog.add_log(vec![LogType::Drink as i32, 0, item_code, item.heal]);
                            }
                        }
                    }
                }
                None => {}
            }

            let item_damages = inflict_damage.get(use_item.item);
            match item_damages {
                Some(item) => {
                    used_item = false;
                    for mob in targets.iter() {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, item.damage);
                        if entity == *player {
                            let mob_code = codes.get(*mob).unwrap().code;
                            let item_code = codes.get(use_item.item).unwrap().code;
                            gamelog.add_log(vec![LogType::UseItem as i32, 0, item_code, mob_code, item.damage]);
                        }
                        used_item = true;
                    }
                }
                None => {}
            }

            if used_item {
                let consumeable = consumeables.get(use_item.item);
                match consumeable {
                    Some(_) => {
                        entities.delete(use_item.item).expect("Delete item failed");
                        state.add_state(INVENTORY_CHANGE);
                    }
                    None => {}
                }
            }
        }
        wants_use.clear();
    }
}
