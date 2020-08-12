use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;

// struct Position moved to common

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: String,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

