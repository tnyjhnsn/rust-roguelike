use specs::prelude::*;
use crate::{
    MyTurn,
    Faction,
    FactionName,
    Position,
    Map,
    FieldOfView,
    WantsToFlee,
    WantsToApproach,
    PlayerEntity,
    Reaction,
    raws::*,
};

pub struct VisibleAI {}

impl<'a> System<'a> for VisibleAI {
    type SystemData = ( 
        ReadStorage<'a, MyTurn>,
        ReadStorage<'a, Faction>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, WantsToApproach>,
        WriteStorage<'a, WantsToFlee>,
        Entities<'a>,
        ReadExpect<'a, PlayerEntity>,
        ReadStorage<'a, FieldOfView>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (turns, factions, positions, map, mut want_approach, mut want_flee,
            entities, player_entity, fovs) = data;

        for (entity, _turn, my_faction, pos, fov) in (&entities, &turns,
            &factions, &positions, &fovs).join() {

            if entity != *player_entity {
                let my_idx = map.xy_idx(pos.x, pos.y);
                let mut reactions = Vec::new();
                let mut flee = Vec::new();
                for visible_tile in &fov.visible_tiles {
                    let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                    if my_idx != idx {
                        evaluate(idx, &map, &factions, &my_faction.name, &mut reactions);
                    }
                }

                let mut done = false;
                for reaction in &reactions {
                    match reaction.1 {
                        Reaction::Attack => {
                            want_approach
                                .insert(entity, WantsToApproach { idx: reaction.0 as i32 })
                                .expect("Unable to insert");
                            done = true;
                        }
                        Reaction::Flee => {
                            flee.push(reaction.0);
                        }
                        _ => {}
                    }
                }

                if !done && !flee.is_empty() {
                    want_flee
                        .insert(entity, WantsToFlee { indices: flee })
                        .expect("Unable to insert");
                }
            }
        }
    }
}

fn evaluate(idx: usize, map: &Map, factions: &ReadStorage<Faction>, my_faction: &FactionName,
    reactions: &mut Vec<(usize, Reaction)>) {

    for other_entity in &map.contents[idx] {
        if let Some(faction) = factions.get(*other_entity) {
            reactions.push((
                idx, 
                get_faction_reaction(&RAWS.lock().unwrap(), my_faction, &faction.name)
            ));
        }
    }
}

