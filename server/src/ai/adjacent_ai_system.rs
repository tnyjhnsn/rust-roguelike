use specs::prelude::*;
use crate::{
    MyTurn,
    Faction,
    FactionName,
    Position,
    Map,
    WantsToMelee,
    Reaction,
    PlayerEntity,
    raws::*,
};

pub struct AdjacentAI {}

impl<'a> System<'a> for AdjacentAI {
    type SystemData = ( 
        WriteStorage<'a, MyTurn>,
        ReadStorage<'a, Faction>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, WantsToMelee>,
        Entities<'a>,
        ReadExpect<'a, PlayerEntity>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, factions, positions, map, mut wants_melee, entities,
            player_entity) = data;

        let mut turn_done = Vec::new();
        for (entity, _turn, my_faction, pos) in (&entities, &turns, &factions,
            &positions).join() {

            if entity != *player_entity {
                let mut reactions = Vec::new();
                let idx = map.xy_idx(pos.x, pos.y);
                for n in &map.neighbours[idx] {
                    evaluate(*n, &map, &factions, &my_faction.name, &mut reactions);
                }

                let mut done = false;
                for reaction in &reactions {
                    if let Reaction::Attack = reaction.1 {
                        wants_melee.insert(entity, WantsToMelee { target: reaction.0 })
                            .expect("Error inserting Melee");
                        done = true;
                    }
                }
                if done {
                    turn_done.push(entity);
                }
            }
        }

        for done in &turn_done {
            turns.remove(*done);
        }
    }
}

fn evaluate(idx: usize, map: &Map, factions: &ReadStorage<Faction>, my_faction: &FactionName,
    reactions: &mut Vec<(Entity, Reaction)>) {

    for other_entity in &map.contents[idx] {
        if let Some(faction) = factions.get(*other_entity) {
            reactions.push((
                *other_entity, 
                get_faction_reaction(&RAWS.lock().unwrap(), my_faction, &faction.name)
            ));
        }
    }
}
