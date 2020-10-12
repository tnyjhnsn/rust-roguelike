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
        ReadExpect<'a, Map>,
        WriteStorage<'a, FieldOfView>, 
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut fov, pos, monsters, entities) = data;
        for (entity, fov, pos) in (&entities, &mut fov, &pos).join() {
            fov.visible_tiles.clear();
            let mut possible_fov = get_possible_fov(pos.x, pos.y, fov.range);
            possible_fov.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            let monster = monsters.get(entity);
            match monster {
                Some(_m) => {
                    fov.visible_tiles = possible_fov;
                }
                None => {
                    let mut set: HashSet<Position> = HashSet::new();
                    for point in &possible_fov {
                        for (x, y) in Bresenham::new((pos.x, pos.y), (point.x, point.y)) {
                            set.insert(Position::new(x, y));
                            let idx = map.xy_idx(x, y);
                            if map.tiles[idx] == TileType::Wall {
                                break;
                            }
                        }
                    }
                    fov.visible_tiles = Vec::from_iter(set);
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

