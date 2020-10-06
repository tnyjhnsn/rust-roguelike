use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;

use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BlocksTile {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Monster {}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct HealthStats {
    pub max_hp: i32,
    pub hp: i32,
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

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Item {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ProvidesHealing {
    pub heal: i32,
}

#[derive(Component, Debug)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Consumeable {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum ArmourSlot {
    Melee = 1,
    Shield = 2,
    Helmet = 3,
    Body = 4,
    Boots = 5,
    Gloves = 6,
    Pendant = 7,
    Ring1 = 8,
    Ring2 = 9,
    Ring3 = 10,
    Ring4 = 11,
    Ring5 = 12,
    Ring6 = 13,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Equippable {
    pub slot: ArmourSlot,
}

#[derive(Component, Debug, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: ArmourSlot,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct DefenseBonus {
    pub defense: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct EntryTrigger {}

#[derive(Component, Debug, Clone)]
pub struct EntityMoved {}

#[derive(Component, Debug, Clone)]
pub struct OtherLevelPosition {
    pub x: i32,
    pub y: i32,
    pub key: &'static str,
}

// Component for each intent

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub item: Entity,
    pub collected_by: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToDropItem {
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToRemoveItem {
    pub item: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToUseItem {
    pub item: Entity,
    pub target: Option<usize>,
}

