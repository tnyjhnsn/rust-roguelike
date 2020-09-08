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
    );

    fn run(&mut self, (map, mut fov, pos): Self::SystemData) {
        for (fov, pos) in (&mut fov, &pos).join() {
            fov.visible_tiles.clear();
            let mut possible_fov = get_possible_fov(pos.x, pos.y, fov.range);
            possible_fov.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            let mut set: HashSet<Point> = HashSet::new();
            for point in &possible_fov {
                for (x, y) in Bresenham::new((pos.x, pos.y), (point.x, point.y)) {
                    set.insert(Point::new(x, y));
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

fn get_possible_fov(x: i32, y: i32, r: i32) -> Vec<Point> {
    let mut v = Vec::new();
    for i in r * -1..r + 1 {
        let mut n = (((r*r - i*i) as f64).sqrt()) as i32;
        n = match i {
            -1 | 1 => n + 1,
            _ => n,
        };
        n = if n == 0 { 1 } else { n };
        for j in n * -1..n + 1 {
            v.push(Point { x: x + i, y: y + j })
        }
    }
    v
}

