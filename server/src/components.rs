use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: String,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

