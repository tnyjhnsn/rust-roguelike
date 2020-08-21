use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;

// consts for binary operations
pub const SEEN: i32 = 1;
pub const VISIBLE: i32 = 2;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: Point) -> f64 {
        let d = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64;
        d.sqrt()
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TileType {
    Wall = 0,
    Floor = 1,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LogType {
    General = 0,
    Player = 1,
    Monster = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMsg {
    pub msg: String,
    pub data: Value,
}

pub type Fov = Vec<(TileType, Vec<usize>)>;
pub type Entities = Vec<(usize, Vec<String>)>;
pub type Logs = Vec<(LogType, String)>;

