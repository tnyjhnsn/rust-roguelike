use specs::prelude::*;
use specs_derive::*;
use roguelike_common::*;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BlocksTile {}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Monster {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Bystander {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Vendor {}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub max: i32,
    pub current: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Pools {
    pub hp: Pool,
    pub mana: Pool,
    pub xp: i32,
    pub level: i32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub base: i32,
    pub modifiers: i32,
    pub bonus: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub might: Attribute,
    pub fitness: Attribute,
    pub quickness: Attribute,
    pub intelligence: Attribute,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Skill {
    Melee,
    Defense,
    Magic,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub skills: HashMap<Skill, i32>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct FieldOfView {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Code {
    pub code: i32,
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

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum EquipmentSlot {
    Melee = 1,
    Shield = 2,
    Head = 3,
    Body = 4,
    Legs = 5,
    Feet = 6,
    Hands = 7,
    Neck = 8,
    Fingers = 9,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Equippable {
    pub slot: EquipmentSlot,
}

#[derive(Component, Debug, Clone)]
pub struct Equipped {
    pub owner: Entity,
    pub slot: EquipmentSlot,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum WeaponAttribute {
    Might,
    Quickness,
}

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum HitDesc {
    Attack,
    CutStab,
    Hit,
    Claw,
    Bite,
    Sting,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct MeleeWeapon {
    pub range: i32,
    pub hit_desc: HitDesc,
    pub attribute: WeaponAttribute,
    pub damage_dice: (i32, i32, i32),
    pub hit_bonus: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NaturalAttackDefense {
    pub armour_class: Option<i32>,
    pub attacks: Vec<MeleeWeapon>,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Wearable {
    pub slot: EquipmentSlot,
    pub armour_class: f32,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LootTableKey {
    Animal,
}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub key: LootTableKey,
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

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BlocksVisibility {}

#[derive(Component, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Door {
    pub open: bool,
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

