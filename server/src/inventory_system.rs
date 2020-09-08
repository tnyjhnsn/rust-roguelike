use specs::prelude::*;
use super::{
    WantsToPickupItem,
    WantsToDropItem,
    WantsToUseItem,
    WantsToRemoveItem,
    Code,
    Map,
    InInventory,
    Position,
    GameLog,
    Consumeable,
    ProvidesHealing,
    InflictsDamage,
    AreaOfEffect,
    Confusion,
    Equippable,
    Equipped,
    SufferDamage,
    HealthStats,
    RunState,
};
use roguelike_common::*;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct PickupItemSystem {}

impl<'a> System<'a> for PickupItemSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
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
    type SystemData = (
        ReadExpect<'a, Entity>,
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
    type SystemData = (
        ReadExpect<'a, Entity>,
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
        WriteStorage<'a, Confusion>,
        ReadStorage<'a, Equippable>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InInventory>,
        WriteExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, map, entities, mut wants_use, codes, consumeables,
             healing, inflict_damage, mut suffer_damage, mut health_stats, aoe,
             mut confused, equippable, mut equipped, mut inventory, mut state) = data;

        for (entity, use_item) in (&entities, &wants_use).join() {

            let item_code = codes.get(use_item.item).unwrap().code;

            let mut targets = Vec::new();
            match use_item.target {
                Some(_t) => {
                    let area_effect = aoe.get(use_item.item);
                    match area_effect {
                        Some(area) => {
                            let idx = use_item.target.unwrap();
                            let mut aoe = vec![idx];
                            map.get_area_of_effect(&mut aoe, area.radius);
                            let aoe_tiles: HashSet<i32> = HashSet::from_iter(aoe);
                            for tile in &aoe_tiles {
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

            let item_equippable = equippable.get(use_item.item);
            match item_equippable {
                None => {}
                Some(can_equip) => {
                    let target_slot = can_equip.slot;
                    let target = targets[0];

                    let mut to_unequip = Vec::new();
                    for (item, already_equipped) in (&entities, &equipped).join() {
                        if already_equipped.owner == target && already_equipped.slot == target_slot {
                            to_unequip.push(item);
                            if target == *player {
                                gamelog.add_log(vec![LogType::Unequip as i32, 0, item_code]);
                                state.add_state(ARMOUR_CHANGE);
                                state.add_state(INVENTORY_CHANGE);
                            }
                        }
                    }
                    for item in &to_unequip {
                        equipped.remove(*item);
                        inventory.insert(*item, InInventory { owner: target })
                            .expect("Unable to insert inventory entry");
                    }
                    equipped.insert(use_item.item, Equipped { owner: target, slot: target_slot })
                        .expect("Unable to insert equipped component");
                    inventory.remove(use_item.item);
                    if target == *player {
                        gamelog.add_log(vec![LogType::Equip as i32, 0, item_code]);
                        state.add_state(ARMOUR_CHANGE);
                        state.add_state(INVENTORY_CHANGE);
                    }
                }
            }

            let item_heals = healing.get(use_item.item);
            match item_heals {
                Some(item) => {
                    for target in &targets {
                        let stats = health_stats.get_mut(*target);
                        if let Some(stats) = stats {
                            stats.hp = i32::min(stats.max_hp, stats.hp + item.heal);
                            if entity == *player {
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
                    for mob in &targets {
                        SufferDamage::new_damage(&mut suffer_damage, *mob, item.damage);
                        if entity == *player {
                            let mob_code = codes.get(*mob).unwrap().code;
                            gamelog.add_log(vec![LogType::UseItem as i32, 0, item_code, mob_code, item.damage]);
                        }
                    }
                }
                None => {}
            }

            let mut add_confusion = Vec::new();
            {
                let causes_confusion = confused.get(use_item.item);
                match causes_confusion {
                    Some(confusion) => {
                        for mob in &targets {
                            add_confusion.push((*mob, confusion.turns));
                            if entity == *player {
                                let mob_code = codes.get(*mob).unwrap().code;
                                gamelog.add_log(vec![LogType::Confusion as i32, 0, item_code, mob_code]);
                            }
                        }
                    }
                    None => {}
                }
            }
            for mob in &add_confusion {
                confused.insert(mob.0, Confusion { turns: mob.1 }).expect("Unable to insert confusion status");
            }

            let consumeable = consumeables.get(use_item.item);
            match consumeable {
                Some(_) => {
                    entities.delete(use_item.item).expect("Delete item failed");
                    state.add_state(INVENTORY_CHANGE);
                }
                None => {}
            }
        }
        wants_use.clear();
    }
}

pub struct RemoveItemSystem {}

impl<'a> System<'a> for RemoveItemSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Code>,
        WriteStorage<'a, WantsToRemoveItem>,
        WriteStorage<'a, Equipped>,
        WriteStorage<'a, InInventory>,
        WriteExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, entities, mut gamelog, codes, mut wants_remove, mut equipped,
             mut inventory, mut state) = data;

        for (entity, to_remove) in (&entities, &wants_remove).join() {
            equipped.remove(to_remove.item);
            inventory.insert(to_remove.item, InInventory { owner: entity }).expect("Unable to insert inventory");
            
            if entity == *player {
                let item_code = codes.get(to_remove.item).unwrap().code;
                gamelog.add_log(vec![LogType::Remove as i32, 0, item_code]);
                state.add_state(ARMOUR_CHANGE);
                state.add_state(INVENTORY_CHANGE);
            }
        }

        wants_remove.clear();
    }
}
