use roguelike_common::*;
use serde::{Serialize, Deserialize};
use super::{Action};

#[derive(Serialize)]
struct Map {
    width: i32,
    height: i32,
    tiles: Vec<TileType>
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 60*20];

    for x in 0..60 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 19)] = TileType::Wall;
    }
    for y in 0..20 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(59, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..200 {
        let x = rng.roll_dice(1, 59);
        let y = rng.roll_dice(1, 19);
        let idx = xy_idx(x, y);
        if idx != xy_idx(20, 10) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn get_position(x: i32, y: i32) -> String {
    let p = Position { x, y };
    let action = Action {
        msg: String::from("POSITION"),
        data: serde_json::to_value(p).unwrap(),
    };
    println!("getting position");
    serde_json::to_string(&action).unwrap()
}

pub fn draw_map(tiles: Vec<TileType>) -> String {
    let map = Map { 
        width: 60,
        height: 20,
        tiles,
    };
    let action = Action {
        msg: String::from("GAME"),
        data: serde_json::to_value(map).unwrap(),
    };
    serde_json::to_string(&action).unwrap()
}
