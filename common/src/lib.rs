use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

// consts for binary operations
pub const SEEN: i32 = 1;
pub const VISIBLE: i32 = 2;
pub const TARGETED: i32 = 4;

pub const WAITING: i32 = 0;
pub const FOV_CHANGE: i32 = 1;
pub const CONTENTS_CHANGE: i32 = 2;
pub const INVENTORY_CHANGE: i32 = 4;
pub const ARMOUR_CHANGE: i32 = 8;
pub const EXIT_MAP: i32 = 16;
pub const GAME_OVER: i32 = 32;

// key codes
// sections
pub const KEY_ESC: u32 = 27;
pub const KEY_I: u32 = 73;
pub const KEY_A: u32 = 65;
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
pub const KEY_R: u32 = 82;
pub const KEY_ENTER: u32 = 13;
// stairs
pub const KEY_GT: u32 = 190;
pub const KEY_LT: u32 = 188;

pub type PlayerEntity = Entity;

pub struct RandomNumberGenerator {
    rng: XorShiftRng,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        let rng: XorShiftRng = SeedableRng::from_entropy();
        Self { rng }
    }
    pub fn roll_dice(&mut self, n: i32, d: i32, m: i32) -> i32 {
        (0..n).map(|_| self.range(1, d + 1)).sum::<i32>() + m
    }
    pub fn range<T>(&mut self, min: T, max: T) -> T
    where T: rand::distributions::uniform::SampleUniform,
    {
        self.rng.gen_range(min, max)
    }
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, other: Position) -> f64 {
        let d = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64;
        d.sqrt()
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum TileType {
    Floor = 0, // Can see through, Can traverse
    Wall = 1, // Cannot see through, Cannot traverse
    Blocked = 2, // Can see through, Cannot traverse
    DownStairs = 3,
    ExitMap = 49, 
    Chasm = 50,
    DeepWater = 51,
    Lava = 52,
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
    Destroyed = 7,
    Confusion = 8,
    Unequip = 9,
    Equip = 10,
    Remove = 11,
    Trap = 12,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMsg {
    pub data: Value,
}

