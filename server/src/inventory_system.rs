use specs::prelude::*;
use super::{WantsToPickupItem, Code, InInventory, Position, GameLog};
use roguelike_common::*;

pub struct InventorySystem {}

impl<'a> System<'a> for InventorySystem {
    type SystemData = (ReadExpect<'a, Entity>,
                       WriteExpect<'a, GameLog>,
                       WriteStorage<'a, WantsToPickupItem>,
                       WriteStorage<'a, Position>,
                       ReadStorage<'a, Code>,
                       WriteStorage<'a, InInventory>,
                       );

    fn run(&mut self, data: Self::SystemData) {
        let (player, mut gamelog, mut wants_pickup, mut positions, codes, mut inventory) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            inventory.insert(pickup.item,
                InInventory { owner: pickup.collected_by }).expect("Unable to insert Inventory entry");

            if pickup.collected_by == *player {
                let item_code = codes.get(pickup.item).unwrap().code;
                gamelog.add_log(vec![LogType::Collect as i32, 0, item_code]);
            }
        }
        wants_pickup.clear();
    }
}

