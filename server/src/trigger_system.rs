use specs::prelude::*;
use super::{
    Campaign,
    EntryTrigger,
    EntityMoved,
    Position,
    Code,
    InflictsDamage,
    SufferDamage,
    GameLog,
    LogType,
    RunState,
};
use roguelike_common::*;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        WriteExpect<'a, Campaign>,
        ReadStorage<'a, EntryTrigger>,
        ReadStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Code>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
        WriteExpect<'a, RunState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut campaign, entry_trigger, entity_moved, position, codes, entities,
             mut gamelog, inflict_damage, mut suffer_damage, mut state) = data;

        let map = campaign.get_active_map();

        for (entity, _entity_moved, pos) in (&entities, &entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for entity_id in map.contents[idx].iter() {
                if entity != *entity_id {
                    let trigger = entry_trigger.get(*entity_id);
                    if let Some(_t) = trigger {
                        let triggerer = codes.get(entity).unwrap().code;
                        let the_trigger = codes.get(*entity_id).unwrap().code;
                        let damage = inflict_damage.get(*entity_id);
                        if let Some(damage) = damage {
                            SufferDamage::new_damage(&mut suffer_damage, entity, damage.damage);
                            gamelog.add_log(vec![LogType::Trap as i32, triggerer, the_trigger, damage.damage]);
                        }
                        if the_trigger == 4999 && triggerer == 0 {
                            println!("here i am");
                            state.add_state(EXIT_MAP);
                        }
                    }
                }
            }
        }
    }
}
