use specs::prelude::*;
use super::{
    EntryTrigger,
    Code,
    InflictsDamage,
    SufferDamage,
    GameLog,
    LogType,
};

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        WriteStorage<'a, EntryTrigger>,
        ReadStorage<'a, Code>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, InflictsDamage>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut entry_trigger, codes, entities, mut gamelog, inflict_damage, mut suffer_damage) = data;

        for (entity, trigger) in (&entities, &mut entry_trigger).join() {
            match trigger.triggered_by {
                None => {},
                Some(e) => {
                    let damage = inflict_damage.get(entity).unwrap().damage;
                    let code = codes.get(entity).unwrap().code;
                    SufferDamage::new_damage(&mut suffer_damage, e, 1000);
                    gamelog.add_log(vec![LogType::Trap as i32, 0, code, damage]);
                    trigger.triggered_by = None;
                }
            };
        }
    }
}
