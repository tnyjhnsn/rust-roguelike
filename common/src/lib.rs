//use std::cmp::{min, max};
use serde_repr::*;

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TileType {
    Wall = 0,
    Floor = 1,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 60) + x as usize
}
