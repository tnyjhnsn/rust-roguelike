use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;

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
pub struct Code {
    pub code: i32,
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct ProvidesHealing {
    pub heal: i32,
}

#[derive(Component, Debug)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Component, Debug)]
pub struct Consumeable {}

#[derive(Component, Debug)]
pub struct Ranged {
    pub range: i32,
}

#[derive(Component, Debug)]
pub struct AreaOfEffect {
    pub radius: i32,
}

#[derive(Component, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

#[derive(Component, Debug)]
pub struct Confusion {
    pub turns: i32,
}

#[derive(PartialEq, Debug, Copy, Clone)]
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

#[derive(Component, Debug, Clone)]
pub struct Equippable {
    pub slot: ArmourSlot,
}

#[derive(Component, Debug, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: ArmourSlot,
}

#[derive(Component, Clone)]
pub struct MeleePowerBonus {
    pub power: i32,
}

#[derive(Component, Clone)]
pub struct DefenseBonus {
    pub defense: i32,
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
    pub target: Option<i32>,
}

