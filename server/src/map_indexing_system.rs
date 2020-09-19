use specs::prelude::*;
use super::{
    Campaign,
    Position,
    BlocksTile
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Campaign>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut campaign, pos, blockers, entities) = data;

        let map = campaign.get_active_map();

        map.populate_blocked();
        map.clear_contents();
        for (entity, pos) in (&entities, &pos).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            let _e: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_e) = _e {
                map.blocked[idx] = true;
            }
            map.contents[idx].push(entity);
        }
    }
}
