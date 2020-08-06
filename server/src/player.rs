use specs::prelude::*;
use super::{Player};
use std::cmp::{min, max};
use roguelike_common::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[dest_idx] != TileType::Wall {
            pos.x = min(59 , max(0, pos.x + delta_x));
            pos.y = min(19, max(0, pos.y + delta_y));
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
