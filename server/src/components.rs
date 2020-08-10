use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;

// struct Position moved to common

#[derive(Component)]
pub struct Renderable {
    pub glyph: String,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
}
