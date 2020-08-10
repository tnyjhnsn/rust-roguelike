use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub type EntityPositions = Vec<(usize, char)>;

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TileType {
    Wall = 0,
    Floor = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMsg {
    pub msg: String,
    pub data: Value,
}

pub type Fov = Vec<(usize, TileType, Vec<String>)>;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 60) + x as usize
}
