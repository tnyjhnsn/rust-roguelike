use specs::prelude::*;
use super::{
    WantsToPickupItem,
    WantsToDropItem,
    WantsToUseItem,
    Code,
    InInventory,
    Position,
    GameLog,
    ProvidesHealing,
    CombatStats,
    RunState,
};
use roguelike_common::*;

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
                        Entities<'a>,
                        WriteStorage<'a, WantsToUseItem>,
                        ReadStorage<'a, Code>,
                        ReadStorage<'a, ProvidesHealing>,
                        WriteStorage<'a, CombatStats>,
                        WriteExpect<'a, RunState>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, entities, mut wants_use, codes, consumeables, mut combat_stats, mut state) = data;

        for (entity, use_item, stats) in (&entities, &wants_use, &mut combat_stats).join() {
            let consumeable = consumeables.get(use_item.item);
            match consumeable {
                None => {}
                Some(item) => {
                    stats.hp = i32::min(stats.max_hp, stats.hp + item.heal);
                    if entity == *player {
                        let item_code = codes.get(use_item.item).unwrap().code;
                        gamelog.add_log(vec![LogType::Drink as i32, 0, item_code, item.heal]);
                    }
                    entities.delete(use_item.item).expect("Delete item failed");
                    state.add_state(INVENTORY_CHANGE);
                }
            }
        }
        wants_use.clear();
    }
}
