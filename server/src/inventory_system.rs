use specs::prelude::*;
use super::{
    WantsToPickupItem,
    WantsToDropItem,
    Code,
    InInventory,
    Position,
    GameLog,
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
