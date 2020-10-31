use specs::prelude::*;
use super::{
    Map,
    Position,
    BlocksTile,
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, pos, tile_blockers, entities) = data;

        map.populate_blocked();
        map.clear_contents();
        for (entity, pos) in (&entities, &pos).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            let _tb: Option<&BlocksTile> = tile_blockers.get(entity);
            if let Some(_tb) = _tb {
                map.blocked[idx] = true;
            }
            map.contents[idx].push(entity);
        }
    }
}
