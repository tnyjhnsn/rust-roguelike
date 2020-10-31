use specs::prelude::*;
use roguelike_common::*;
use super::components::*;
use super::map::*;
use line_drawing::Bresenham;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>, 
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        Entities<'a>,
        ReadStorage<'a, BlocksVisibility>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, mut fov, pos, player, entities, blocks_visibility) = data;

        map.view_blocked.clear();
        for (block_pos, _block) in (&pos, &blocks_visibility).join() {
            let idx = map.xy_idx(block_pos.x, block_pos.y);
            map.view_blocked.insert(idx);
        }

        for (entity, fov, pos) in (&entities, &mut fov, &pos).join() {
            fov.visible_tiles.clear();
            let mut possible_fov = get_possible_fov(pos.x, pos.y, fov.range);
            possible_fov.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            match player.get(entity) {
                Some(_p) => {
                    let mut set: HashSet<Position> = HashSet::new();
                    for point in &possible_fov {
                        for (x, y) in Bresenham::new((pos.x, pos.y), (point.x, point.y)) {
                            set.insert(Position::new(x, y));
                            let idx = map.xy_idx(x, y);
                            if map.tiles[idx] == TileType::Wall || map.view_blocked.contains(&idx) {
                                break;
                            }
                        }
                    }
                    fov.visible_tiles = Vec::from_iter(set);
                }
                None => {
                    fov.visible_tiles = possible_fov;
                }
            }
        }
    }
}

fn get_possible_fov(x: i32, y: i32, r: i32) -> Vec<Position> {
    let mut v = Vec::new();
    for i in r * -1..r + 1 {
        let mut n = (((r*r - i*i) as f64).sqrt()) as i32;
        n = match i {
            -1 | 1 => n + 1,
            _ => n,
        };
        n = if n == 0 { 1 } else { n };
        for j in n * -1..n + 1 {
            v.push(Position { x: x + i, y: y + j })
        }
    }
    v
}

