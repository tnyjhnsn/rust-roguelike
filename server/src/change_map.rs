use specs::prelude::*;
use super::*;

pub fn exit_map(ecs: &World) -> (Map, bool) {
    let mut state = ecs.fetch_mut::<RunState>();
    let mut campaign = ecs.fetch_mut::<Campaign>();
    let mut ppos = ecs.fetch_mut::<PlayerPosition>();
    let (map, new_ppos, visited) = campaign.create_map_from_exit(ppos.position);

    let mut pos = ecs.write_storage::<Position>();
    let player_entity = ecs.fetch::<PlayerEntity>();

    let player_pos = pos.get_mut(*player_entity);
    if let Some(player_pos) = player_pos {
        player_pos.x = new_ppos.x;
        ppos.position.x = new_ppos.x;
        player_pos.y = new_ppos.y;
        ppos.position.y = new_ppos.y;
    }
    state.remove_state(EXIT_MAP);
    (map, visited)
}

pub fn freeze_entities(ecs: &World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<PlayerEntity>();
    let key = ecs.fetch::<Map>().key;

    let mut pos_to_delete = Vec::new();
    for (entity, pos) in (&entities, &positions).join() {
        if entity != *player_entity {
            other_level_positions.insert(entity,
                OtherLevelPosition { x: pos.x, y: pos.y, key })
                    .expect("Cannot freeze entities");
            pos_to_delete.push(entity);
        }
    }
    for p in pos_to_delete.iter() {
        positions.remove(*p);
    }
}

pub fn thaw_entities(key: &str, ecs: &World) {
    let entities = ecs.entities();
    let mut positions = ecs.write_storage::<Position>();
    let mut other_level_positions = ecs.write_storage::<OtherLevelPosition>();
    let player_entity = ecs.fetch::<Entity>();

    let mut pos_to_delete = Vec::new();
    for (entity, pos) in (&entities, &other_level_positions).join() {
        if entity != *player_entity && pos.key == key {
            positions.insert(entity, Position { x: pos.x, y: pos.y })
                .expect("Cannot thaw entities");
            pos_to_delete.push(entity);
        }
    }

    for p in pos_to_delete.iter() {
        other_level_positions.remove(*p);
    }
}

