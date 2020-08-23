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

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let damage = SufferDamage { amount: vec![amount] };
            store.insert(victim, damage).expect("Unable to insert damage");
        }
    }
}

// Component for each intent

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

