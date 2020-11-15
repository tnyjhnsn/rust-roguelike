use specs::prelude::*;
use super::{
    Player,
    Item,
    GameLog,
    WantsToMelee,
    WantsToPickupItem,
    WantsToDropItem,
    WantsToUseItem,
    InInventory,
    Equipped,
    WantsToRemoveItem,
    GuiState,
    GameState,
    EntityMoved,
    Map,
    Particles,
    Door,
    BlocksVisibility,
    BlocksTile,
    Faction,
    FactionName,
    Attributes,
    Reaction,
    Vendor,
    VendorDialog,
    raws::*,
};
use std::cmp::{min, max};
use roguelike_common::*;

#[derive(Debug)]
pub struct PlayerPosition {
    pub position: Position,
}

impl PlayerPosition {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> GameState {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let combat_stats = ecs.read_storage::<Attributes>();
    let map = ecs.fetch::<Map>();
    let mut gui_state = ecs.fetch_mut::<GuiState>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let mut entity_moved = ecs.write_storage::<EntityMoved>();
    let mut particles = ecs.fetch_mut::<Particles>();
    let mut doors = ecs.write_storage::<Door>();
    let mut blocks_visibility = ecs.write_storage::<BlocksVisibility>();
    let mut blocks_tile = ecs.write_storage::<BlocksTile>();
    let factions = ecs.read_storage::<Faction>();
    let vendors = ecs.read_storage::<Vendor>();
    let mut dialog = ecs.fetch_mut::<VendorDialog>();

    let mut game_state = GameState::Ticking;
    let mut swap_entities = Vec::new();

    for (entity, _player, pos) in (&entities, &mut players, &mut positions).join() {
        if pos.x + delta_x < 0 || pos.x + delta_x > map.width - 1
            || pos.y + delta_y < 0 || pos.y + delta_y > map.height - 1 { return game_state; }
        let dest_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        game_state = crate::spatial::for_each_tile_content_with_gamemode(dest_idx, |potential_target| {

            if let Some(vendor) = vendors.get(potential_target) {
                let categories = get_vendor_items(
                    &RAWS.lock().unwrap(),
                    &vendor.categories,
                );
                // TODO Testing - need to use proper entity code
                dialog.add_dialog((999, categories));
                return Some(game_state);
            }

            let mut hostile = true;
            if combat_stats.get(potential_target).is_some() {
                if let Some(faction) = factions.get(potential_target) {
                    let reaction = get_faction_reaction(
                        &RAWS.lock().unwrap(),
                        &faction.name,
                        &FactionName::Player,
                    );
                    if reaction != Reaction::Attack { hostile = false; }
                }
            }
            if !hostile {
                swap_entities.push((potential_target, pos.x, pos.y));
                pos.x = min(map.width - 1 , max(0, pos.x + delta_x));
                pos.y = min(map.height - 1, max(0, pos.y + delta_y));
                let mut ppos = ecs.write_resource::<PlayerPosition>();
                ppos.position.x = pos.x;
                ppos.position.y = pos.y;
                entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert marker");
                gui_state.add_state(FOV_CHANGE);
                gui_state.add_state(CONTENTS_CHANGE);
                return Some(game_state);
            } else {
                let target = combat_stats.get(potential_target);
                if let Some(_target) = target {
                    wants_to_melee.insert(entity, WantsToMelee{ target: potential_target }).expect("Add target failed");
                    particles.add_particle((PARTICLE_ATTACK, vec![map.xy_idx(pos.x, pos.y)]));
                    particles.add_particle((PARTICLE_DEFEND, vec![dest_idx]));
                    return Some(game_state);
                }
            }
            let door = doors.get_mut(potential_target);
            if let Some(door) = door {
                door.open = true;
                blocks_visibility.remove(potential_target);
                blocks_tile.remove(potential_target);
                gui_state.add_state(FOV_CHANGE);
                return Some(game_state);
            }
            None
        });

        if !crate::spatial::is_blocked(dest_idx) {
            let old_idx = map.xy_idx(pos.x, pos.y);
            pos.x = min(map.width - 1 , max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            let new_idx = map.xy_idx(pos.x, pos.y);
            entity_moved.insert(entity, EntityMoved {}).expect("Unable to insert move");
            crate::spatial::move_entity(entity, old_idx, new_idx);

            let mut ppos = ecs.write_resource::<PlayerPosition>();
            ppos.position.x = pos.x;
            ppos.position.y = pos.y;
            game_state = GameState::Ticking;
            match map.tiles[dest_idx] {
                TileType::ExitMap => game_state = GameState::ExitMap,
                _ => {}
            }
            gui_state.add_state(FOV_CHANGE);
            gui_state.add_state(CONTENTS_CHANGE);
        }
    }

    for e in swap_entities {
        let their_pos = positions.get_mut(e.0);
        if let Some(their_pos) = their_pos {
            let old_idx = map.xy_idx(their_pos.x, their_pos.y);
            their_pos.x = e.1;
            their_pos.y = e.2;
            let new_idx = map.xy_idx(their_pos.x, their_pos.y);
            crate::spatial::move_entity(e.0, old_idx, new_idx);
            game_state = GameState::Ticking;
        }
    }

    game_state
}

pub fn player_input(txt: String, ecs: &mut World) {
    let result = match txt.trim() {
        "ArrowLeft" => try_move_player(-1, 0, ecs),
        "ArrowRight" => try_move_player(1, 0, ecs),
        "ArrowUp" => try_move_player(0, -1, ecs),
        "ArrowDown" => try_move_player(0, 1, ecs),
        "Y"|"y" => try_move_player(-1, -1, ecs),
        "U"|"u" => try_move_player(1, -1, ecs),
        "N"|"n" => try_move_player(1, 1, ecs),
        "B"|"b" => try_move_player(-1, 1, ecs),
        _ => GameState::Ticking,
    };
    let mut game_state = ecs.write_resource::<GameState>();
    *game_state = result;
}

pub fn pickup_item(ecs: &mut World) {

    let ppos = ecs.fetch::<PlayerPosition>();
    let player = ecs.fetch::<PlayerEntity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();
    let mut gamelog = ecs.fetch_mut::<GameLog>();    
    let mut game_state = ecs.write_resource::<GameState>();

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
    *game_state = GameState::Ticking;
}

pub fn drop_item(idx: i32, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let inventory = ecs.read_storage::<InInventory>();
    let entities = ecs.entities();
    let mut game_state = ecs.write_resource::<GameState>();

    for (entity, _i) in (&entities, &inventory).join().filter(|item| item.0.id() as i32 == idx) {
        let mut intent = ecs.write_storage::<WantsToDropItem>();
        intent.insert(*player,
            WantsToDropItem{ item: entity }).expect("Unable to insert wants to drop item");
    }
    *game_state = GameState::Ticking;
}

pub fn remove_item(idx: i32, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let equipped = ecs.read_storage::<Equipped>();
    let entities = ecs.entities();
    let mut game_state = ecs.write_resource::<GameState>();

    for (entity, _i) in (&entities, &equipped).join().filter(|item| item.0.id() as i32 == idx) {
        let mut intent = ecs.write_storage::<WantsToRemoveItem>();
        intent.insert(*player,
            WantsToRemoveItem{ item: entity }).expect("Unable to insert wants to remove item");
    }
    *game_state = GameState::Ticking;
}

pub fn use_item(idx: i32, target: Option<usize>, ecs: &mut World) {
    let player = ecs.fetch::<PlayerEntity>();
    let inventory = ecs.read_storage::<InInventory>();
    let entities = ecs.entities();
    let mut game_state = ecs.write_resource::<GameState>();

    for (entity, _i) in (&entities, &inventory).join().filter(|item| item.0.id() as i32 == idx) {
        let mut intent = ecs.write_storage::<WantsToUseItem>();
        intent.insert(*player,
            WantsToUseItem{ item: entity, target  }).expect("Unable to insert wants to use item");
    }
    *game_state = GameState::Ticking;
}
