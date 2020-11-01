use specs::prelude::*;
use crate::{
    MyTurn,
    WantsToApproach,
    Position, 
    Map,
    EntityMoved,
    GuiState,
    lowest_exit,
};
use roguelike_common::*;

pub struct ApproachAI {}

impl<'a> System<'a> for ApproachAI {
    type SystemData = ( 
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, WantsToApproach>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, EntityMoved>,
        Entities<'a>,
        WriteExpect<'a, GuiState>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut want_approach, mut positions, mut map, 
            mut entity_moved, entities, mut gui_state) = data;
            
        let mut turn_done = Vec::new();
        for (entity, mut pos, approach, _myturn) in 
            (&entities, &mut positions, &want_approach, &turns).join() {

            turn_done.push(entity);

            let idx = map.xy_idx(pos.x, pos.y);
            let target_pos = map.idx_xy(approach.idx);
            let dijkstra_map = create_dijkstra_map(target_pos.x, target_pos.y, &map);
            let new_pos = map.dijkstra_exit(&dijkstra_map, pos.x, pos.y, lowest_exit);
            pos.x = new_pos.x;
            pos.y = new_pos.y;
            entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert move");
            let new_idx = map.xy_idx(pos.x, pos.y);
            crate::spatial::move_entity(entity, idx, new_idx);
            gui_state.add_state(CONTENTS_CHANGE);
        }

        want_approach.clear();

        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}

const DIJKSTRA_RANGE: i32 = 10;

fn create_dijkstra_map(x: i32, y: i32, map: &Map) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..=DIJKSTRA_RANGE {
        for a in [-1, 1].iter().cloned() {
            for j in 0..=DIJKSTRA_RANGE {
                for b in [-1, 1].iter().cloned() {
                    if (i == 0 && a == -1) || (j == 0 && b == -1) {
                        continue;
                    }
                    let xp = x + a * i;
                    let yp = y + b * j;
                    if xp < 0 || xp >= map.width || yp < 0 || yp >= map.height {
                        continue;
                    }
                    let idx = map.xy_idx(xp, yp);
                    if !crate::spatial::is_blocked(idx) {
                        v.push(idx)
                    }
                };
            };
        };
    };
    v
}
