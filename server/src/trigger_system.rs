use specs::prelude::*;
use super::{
    Map,
    EntryTrigger,
    EntityMoved,
    Position,
    Code,
    InflictsDamage,
    SufferDamage,
    GameLog,
    LogType,
};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, EntryTrigger>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Code>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, entry_trigger, mut entity_moved, position, codes, entities,
             mut gamelog, inflict_damage, mut suffer_damage) = data;

        for (entity, _entity_moved, pos) in (&entities, &entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            crate::spatial::for_each_tile_content(idx, |entity_id| {
                if entity != entity_id {
                    let trigger = entry_trigger.get(entity_id);
                    if let Some(_t) = trigger {
                        let triggerer = codes.get(entity).unwrap().code;
                        let the_trigger = codes.get(entity_id).unwrap().code;
                        let damage = inflict_damage.get(entity_id);
                        if let Some(damage) = damage {
                            SufferDamage::new_damage(&mut suffer_damage, entity, damage.damage, false);
                            gamelog.add_log(vec![LogType::Trap as i32, triggerer, the_trigger, damage.damage]);
                        }
                    }
                }
            });
        }
        entity_moved.clear();
    }
}
