use specs::prelude::*;
use super::{FieldOfView, Monster, Name};
use roguelike_common::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, FieldOfView>, 
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, fov, monster, name) = data;

        for (fov, _monster, name) in (&fov, &monster, &name).join() {
            if fov.visible_tiles.contains(&*player_pos) {
                println!("{} shouts insults!", name.name);
            }

        }
    }
}

