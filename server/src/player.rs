use specs::prelude::*;
use super::{
    Player,
    Item,
    GameLog,
    CombatStats,
    WantsToMelee,
    WantsToPickupItem,
    WantsToDropItem,
    WantsToUseItem,
    InInventory,
    Equipped,
    WantsToRemoveItem,
    RunState,
    EntryTrigger,
};
use std::cmp::{min, max};
use roguelike_common::*;

use super::map::*;

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: Point,
}

impl PlayerPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Point::new(x, y),
        }
    }
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let mut state = ecs.fetch_mut::<RunState>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos) in (&entities, &mut players, &mut positions).join() {
        if pos.x + delta_x < 1 || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 1 || pos.y + delta_y > map.height - 1 { return; }
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in &map.contents[dest_idx] {
            let t = combat_stats.get(*potential_target);
            match t {
                Some(_t) => {
                    wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target })
                        .expect("Add target failed");
                    return;
                }
                None => {}
            }
        }

        if !map.blocked[dest_idx] {
            pos.x = min(map.width - 1 , max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            let mut ppos = ecs.write_resource::<PlayerPosition>();
            ppos.position.x = pos.x;
            ppos.position.y = pos.y;
            state.add_state(FOV_CHANGE);
            state.add_state(CONTENTS_CHANGE);

            match map.tiles[dest_idx] {
                TileType::Chasm => {
                    let player = ecs.fetch::<PlayerEntity>();
                    let mut entry_trigger = ecs.write_storage::<EntryTrigger>();
                    let chasm = &map.contents[dest_idx][0];
                    if let Some(trap) = entry_trigger.get_mut(*chasm) {
                        trap.triggered_by = Some(*player);
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn try_next_level(ecs: &mut World) -> bool {
    let ppos = ecs.fetch::<PlayerPosition>();
    let map = ecs.fetch::<Map>();
    let ppos_idx = map.xy_idx(ppos.position.x, ppos.position.y);

    if map.tiles[ppos_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog.add_log(vec![LogType::System as i32, 2]);
        false
    }
}

pub fn player_input(txt: String, ecs: &mut World) {
    match txt.trim() {
        "ArrowLeft" => try_move_player(-1, 0, ecs),
        "ArrowRight" => try_move_player(1, 0, ecs),
        "ArrowUp" => try_move_player(0, -1, ecs),
        "ArrowDown" => try_move_player(0, 1, ecs),
        "Y"|"y" => try_move_player(-1, -1, ecs),
        "U"|"u" => try_move_player(1, -1, ecs),
        "N"|"n" => try_move_player(1, 1, ecs),
        "B"|"b" => try_move_player(-1, 1, ecs),
        _ => ()
    }
}

pub fn pickup_item(ecs: &mut World) {

    let ppos = ecs.fetch::<PlayerPosition>();
    let player = ecs.fetch::<PlayerEntity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();    

    let mut target_item: Option<Entity> = None;
    for (item, _i, pos) in (&entities, &items, &positions).join() {
        if pos.x == ppos.position.x && pos.y == ppos.position.y {
            target_item = Some(item);
        }
    }

    match target_item {
        Some(item) => {
            let mut intent = ecs.write_storage::<WantsToPickupItem>();
            intent.insert(*player,
                WantsToPickupItem{ collected_by: *player, item })
                    .expect("Unable to insert wants to pickup item");
        }
        None => gamelog.add_log(vec![LogType::System as i32, 1]),
    }
}

pub fn drop_item(idx: u64, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let inventory = ecs.read_storage::<InInventory>();
    let entities = ecs.entities();

    for (entity, _i) in (&entities, &inventory).join().filter(|item| item.0.id() as u64 == idx) {
        let mut intent = ecs.write_storage::<WantsToDropItem>();
        intent.insert(*player,
            WantsToDropItem{ item: entity }).expect("Unable to insert wants to drop item");
    }
}

pub fn remove_item(idx: u64, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let equipped = ecs.read_storage::<Equipped>();
    let entities = ecs.entities();

    for (entity, _i) in (&entities, &equipped).join().filter(|item| item.0.id() as u64 == idx) {
        let mut intent = ecs.write_storage::<WantsToRemoveItem>();
        intent.insert(*player,
            WantsToRemoveItem{ item: entity }).expect("Unable to insert wants to remove item");
    }
}

pub fn use_item(idx: u64, target: Option<i32>, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let inventory = ecs.read_storage::<InInventory>();
    let entities = ecs.entities();

    for (entity, _i) in (&entities, &inventory).join().filter(|item| item.0.id() as u64 == idx) {
        let mut intent = ecs.write_storage::<WantsToUseItem>();
        intent.insert(*player,
            WantsToUseItem{ item: entity, target  }).expect("Unable to insert wants to use item");
    }
}
