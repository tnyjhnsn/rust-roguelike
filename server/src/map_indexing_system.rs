use specs::prelude::*;
use super::{
    Map,
    Position,
    BlocksTile,
    Pools,
    spatial,
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, Pools>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, pos, blockers, pools, entities) = data;

        spatial::clear();
        spatial::populate_blocked_from_map(&*map);
        for (entity, pos) in (&entities, &pos).join() {
            let mut alive = true;
            if let Some(pools) = pools.get(entity) {
                if pools.hp.current < 1 {
                    alive = false;
                }
            }
            if alive {
                let idx = map.xy_idx(pos.x, pos.y);
                spatial::index_entity(entity, idx, blockers.get(entity).is_some());
            }
        }
    }
}
