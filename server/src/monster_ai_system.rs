use specs::prelude::*;
use super::{FieldOfView, Monster};
use roguelike_common::*;

pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = ( ReadExpect<'a, Point>,
                        ReadStorage<'a, FieldOfView>, 
                        ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData) {
        let (player_pos, fov, monster) = data;

        for (fov, _monster) in (&fov, &monster).join() {
            if fov.visible_tiles.contains(&*player_pos) {
                println!("Monster shouts insults!");
            }

        }
    }
}
