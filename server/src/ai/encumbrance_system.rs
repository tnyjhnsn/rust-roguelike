use specs::prelude::*;
use crate::{
    EquipmentChanged,
    Item,
    InInventory,
    Equipped,
    Pools,
    Attributes,
    GameLog,
    PlayerEntity,
    LogType,
};
use std::collections::HashMap;

pub struct EncumbranceSystem {}

impl<'a> System<'a> for EncumbranceSystem {
    type SystemData = ( 
        WriteStorage<'a, EquipmentChanged>,
        Entities<'a>,
        ReadStorage<'a, Item>,
        ReadStorage<'a, InInventory>,
        ReadStorage<'a, Equipped>,
        WriteStorage<'a, Pools>,
        ReadStorage<'a, Attributes>,
        ReadExpect<'a, PlayerEntity>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut equip_dirty, entities, items, backpacks, wielded, 
            mut pools, attributes, player, mut gamelog) = data;

        if equip_dirty.is_empty() { return; }

        let mut to_update = HashMap::new();
        for (entity, _dirty) in (&entities, &equip_dirty).join() {
            to_update.insert(entity, (0.0, 0.0));
        }

        equip_dirty.clear();

        for (item, equipped) in (&items, &wielded).join() {
            if to_update.contains_key(&equipped.owner) {
                let totals = to_update.get_mut(&equipped.owner).unwrap();
                totals.0 += item.weight_lbs;
                totals.1 += item.initiative_penalty;
            }
        }

        for (item, carried) in (&items, &backpacks).join() {
            if to_update.contains_key(&carried.owner) {
                let totals = to_update.get_mut(&carried.owner).unwrap();
                totals.0 += item.weight_lbs;
                totals.1 += item.initiative_penalty;
            }
        }

        for (entity, (weight, initiative)) in to_update.iter() {
            if let Some(pool) = pools.get_mut(*entity) {
                pool.tot_weight = *weight;
                pool.tot_initiative_penalty = *initiative;

                if let Some(attr) = attributes.get(*entity) {
                    let carry_capacity = ((attr.might.base + attr.might.modifiers) * 15) as f32;
                    pool.carry_capacity = carry_capacity;
                    if pool.tot_weight > carry_capacity {
                        pool.tot_initiative_penalty += 4.0;
                        if *entity == *player {
                            gamelog.add_log(vec![LogType::System as i32, 3]);
                        }
                    }
                }
            }
        }
    }
}
