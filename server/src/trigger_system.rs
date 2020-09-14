use specs::prelude::*;
use super::{
    Position,
    EntryTrigger,
    Map,
    Code,
    InflictsDamage,
    SufferDamage,
    GameLog,
};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntryTrigger>,
        ReadStorage<'a, Code>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, position, entry_trigger, codes, entities, mut gamelog, inflicts_damage,
            mut inflict_damage) = data;

        for (entity, trigger) in (&entities, &entry_trigger).join() {
            match trigger.triggered_by {
                None => {},
                Some(e) => {
                    SufferDamage::new_damage(&mut inflict_damage, e, 1000);
                }
            }
        }
    }
}
