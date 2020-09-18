use specs::prelude::*;
use super::{
    GameSocket,
    Player,
    InInventory,
    Equipped,
    //map::*,
    //spawner::*,
};
use roguelike_common::*;

impl GameSocket {

    pub fn go_downstairs(&mut self) {
        let to_delete = self.entities_to_remove_on_level_change();
        for target in &to_delete {
            self.ecs.delete_entity(*target).expect("Unable to delete entity");
        }
        //let mut new_map = down_stairs(&mut self.ecs);
        //spawn_map(&mut new_map, &mut self.ecs);
        //self.ecs.insert(new_map);
    }

    fn entities_to_remove_on_level_change(&self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let inventory = self.ecs.read_storage::<InInventory>();
        let player_entity = self.ecs.fetch::<PlayerEntity>();
        let equipped = self.ecs.read_storage::<Equipped>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            let p = player.get(entity);
            if let Some(_p) = p {
                should_delete = false;
            }

            let bp = inventory.get(entity);
            if let Some(bp) = bp {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }

            let eq = equipped.get(entity);
            if let Some(eq) = eq {
                if eq.owner == *player_entity {
                    should_delete = false;
                }
            }

            if should_delete { 
                to_delete.push(entity);
            }
        }
        to_delete
    }
}

