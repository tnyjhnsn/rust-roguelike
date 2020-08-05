use specs::prelude::*;
use specs_derive::*;
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
