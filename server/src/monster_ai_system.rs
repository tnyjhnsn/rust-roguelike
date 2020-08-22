use specs::prelude::*;
use roguelike_common::*;
use super::player::*;
use super::gamelog::*;
use super::components::*;
use super::map::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = ( WriteExpect<'a, PlayerPosition>,
                        ReadStorage<'a, FieldOfView>, 
                        WriteExpect<'a, Map>, 
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteExpect<'a, GameLog>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (ppos, fov, mut map, monster, mut mpos, mut log, name) = data;

        for (fov, _monster, mpos, name) in (&fov, &monster, &mut mpos, &name).join() {
            let distance = ppos.position.distance(Point::new(mpos.x, mpos.y));
            if distance < 1.5 {
                println!("{} at {} shouts insults!", name.name, distance);
                log.add_log((LogType::Monster, format!("{} shouts insults", name.name)));
                return;
            } else if fov.visible_tiles.contains(&ppos.position) {
                let mut idx = map.xy_idx(mpos.x, mpos.y);
                map.blocked[idx] = false;
                let dijkstra_map = create_dijkstra_map(ppos.position.x, ppos.position.y, &map);
                let new_pos = map.populate_dijkstra_values(&dijkstra_map, mpos.x, mpos.y);
                mpos.x = new_pos.x;
                mpos.y = new_pos.y;
                idx = map.xy_idx(mpos.x, mpos.y);
                map.blocked[idx] = true;
            }
        }
    }
}

const DIJKSTRA_RANGE: i32 = 7;

fn create_dijkstra_map(x: i32, y: i32, map: &Map) -> Vec<usize> {
    let mut v = Vec::new();
    for i in 0..=DIJKSTRA_RANGE {
        for a in [-1, 1].iter().cloned() {
            for j in 0..=DIJKSTRA_RANGE {
                for b in [-1, 1].iter().cloned() {
                    if (i == 0 && a == -1) || (j == 0 && b == -1) {
                        continue;
                    }
                    let xp = x+a*i;
                    let yp = y+b*j;
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
