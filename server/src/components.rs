use specs::prelude::*;
use specs_derive::*;

// struct Position moved to common

#[derive(Component)]
pub struct Renderable {
    pub glyph: char,
}

#[derive(Component, Debug)]
pub struct Player {}
