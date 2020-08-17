use specs::prelude::*;
use super::{Player};
use std::cmp::{min, max};
use roguelike_common::*;

use super::map::*;

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: Point,
    pub dijkstra_map: Vec<(Position, i32, Vec<Position>)>,
}

impl PlayerPosition {
    pub fn get_next_position(&self, x: i32, y: i32) -> Position {
        let current_pos = self.idx_pos(Position { x, y });
        let current_dv = self.dijkstra_map[current_pos].1;
        for n in self.dijkstra_map[current_pos].2.iter() {
            let idx = self.idx_pos(*n);
            let dv = self.dijkstra_map[idx].1;
            if dv < current_dv {
                return *n;
            }
        };
        Position { x, y }
    }

    fn idx_pos(&self, p: Position) -> usize {
        let idx = self.dijkstra_map.iter().position(|m| m.0.x == p.x && m.0.y == p.y);
        match idx {
            Some(i) => i,
            None => 0,
        }
    }
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[dest_idx] != TileType::Wall {
            pos.x = min(map.width - 1 , max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            let mut ppos = ecs.write_resource::<PlayerPosition>();
            ppos.position.x = pos.x;
            ppos.position.y = pos.y;
            ppos.dijkstra_map = Vec::new();
        }
    }
}

pub fn player_input( txt: String, ecs: &mut World) {
    match txt.trim() {
        "ArrowLeft" => {
            try_move_player(-1, 0, ecs);
        }
        "ArrowRight" => {
            try_move_player(1, 0, ecs);
        }
        "ArrowUp" => {
            try_move_player(0, -1, ecs);
        }
        "ArrowDown" => {
            try_move_player(0, 1, ecs);
        }
        _ => ()
    }
}
