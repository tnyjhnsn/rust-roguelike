use specs::prelude::*;
use super::{
    Map,
    Position,
    BlocksTile,
    BlocksVisibility,
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, BlocksVisibility>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, pos, tile_blockers, view_blockers, entities) = data;

        map.populate_blocked();
        map.clear_contents();
        map.view_blocked.clear();
        for (entity, pos) in (&entities, &pos).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            let _tb: Option<&BlocksTile> = tile_blockers.get(entity);
            if let Some(_tb) = _tb {
                map.blocked[idx] = true;
            }
            let _vb: Option<&BlocksVisibility> = view_blockers.get(entity);
            if let Some(_vb) = _vb {
                map.view_blocked.insert(idx);
            }
            map.contents[idx].push(entity);
        }
    }
}
