use specs::prelude::*;
use roguelike_common::*;
use crate::{
    PlayerPosition,
    FieldOfView,
    Map,
    GuiState,
    Monster,
    Position,
    PlayerEntity,
    WantsToMelee,
    EntityMoved,
    Particles,
    MyTurn,
};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, PlayerPosition>,
        ReadStorage<'a, FieldOfView>, 
        WriteExpect<'a, Map>, 
        WriteExpect<'a, GuiState>, 
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        ReadExpect<'a, PlayerEntity>,
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, EntityMoved>,
        WriteExpect<'a, Particles>,
        WriteStorage<'a, MyTurn>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ppos, fov, mut map, mut gui_state, monster, mut mpos, player_entity,
             entities, mut wants_to_melee, mut entity_moved, mut particles,
             mut turns) = data;

        let mut turn_done = Vec::new();

        for (entity, fov, _m, mpos, _turn) in (&entities, &fov, &monster,
            &mut mpos, &turns).join() {

            turn_done.push(entity);

            let distance = ppos.position.distance(Position::new(mpos.x, mpos.y));
            if distance < 1.5 {
                wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
                particles.add_particle((PARTICLE_ATTACK, vec![map.xy_idx(mpos.x, mpos.y)]));
                particles.add_particle((PARTICLE_DEFEND, vec![map.xy_idx(ppos.position.x, ppos.position.y)]));
            } else if fov.visible_tiles.contains(&ppos.position) {
                println!("chasing...");
                let mut idx = map.xy_idx(mpos.x, mpos.y);
                map.blocked[idx] = false;
                let dijkstra_map = create_dijkstra_map(ppos.position.x, ppos.position.y, &map);
                let new_pos = map.populate_dijkstra_values(&dijkstra_map, mpos.x, mpos.y);
                mpos.x = new_pos.x;
                mpos.y = new_pos.y;
                entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert move");
                idx = map.xy_idx(mpos.x, mpos.y);
                map.blocked[idx] = true;
                gui_state.add_state(CONTENTS_CHANGE);
            }

        }

        for done in &turn_done {
            turns.remove(*done);
        }
    }
}

const DIJKSTRA_RANGE: i32 = 8;

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
                    if !map.blocked[idx] {
                        v.push(idx)
                    }
                };
            };
        };
    };
    v
}
