use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;

// consts for binary operations
pub const SEEN: i32 = 1;
pub const VISIBLE: i32 = 2;
pub const TARGETED: i32 = 4;

pub const WAITING: i32 = 0;
pub const FOV_CHANGE: i32 = 1;
pub const CONTENTS_CHANGE: i32 = 2;
pub const INVENTORY_CHANGE: i32 = 4;

// key codes
// sections
pub const KEY_ESC: u32 = 27;
pub const KEY_I: u32 = 73;
// movement
pub const KEY_LEFT: u32 = 37;
pub const KEY_UP: u32 = 38;
pub const KEY_RIGHT: u32 = 39;
pub const KEY_DOWN: u32 = 40;
pub const KEY_Y: u32 = 89;
pub const KEY_U: u32 = 85;
pub const KEY_B: u32 = 66;
pub const KEY_N: u32 = 78;
pub const KEY_G: u32 = 71;
// actions
pub const KEY_D: u32 = 68;
pub const KEY_ENTER: u32 = 13;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
#[repr(i32)]
pub enum LogType {
    System = 0,
    Melee = 1,
    Dead = 2,
    Collect = 3,
    Drop = 4,
    Drink = 5,
    UseItem = 6,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMsg {
    pub data: Value,
}

pub type Fov = Vec<(TileType, Vec<usize>)>;

