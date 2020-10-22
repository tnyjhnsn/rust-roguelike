use std::collections::{HashMap};
use super::*;

use actix::{
    Actor,
};
use serde_json::json;

#[derive(PartialEq, Copy, Clone)]
pub struct RunState {
    state: i32,
}

impl RunState {
    fn new(state: i32) -> Self {
        Self { state }
    }

    pub fn add_state(&mut self, state: i32) {
        self.state |= state;
    }

    pub fn remove_state(&mut self, state: i32) {
        self.state &= !state;
    }

    pub fn check_state(&self, state: i32) -> bool {
        self.state & state != 0
    }
}

impl GameSocket {

    pub fn new_campaign(&mut self) {

        load_raws();

        let mut campaign = Campaign::new();
        self.ecs.insert(RandomNumberGenerator::new());

        let (mut map, ppos, _visited) = campaign.create_map_from_exit(Position::new(0, 0));
        spawn_map(&mut map, &mut self.ecs);
        
        let player = player(&mut self.ecs, ppos.x, ppos.y);
        self.ecs.insert(player);
        self.ecs.insert(PlayerPosition::new(ppos));

        self.ecs.insert(map);
        self.ecs.insert(campaign);

        let mut state = RunState::new(WAITING);
        state.add_state(INVENTORY_CHANGE);
        state.add_state(ARMOUR_CHANGE);
        state.add_state(COMBAT_STATS_CHANGE);
        state.add_state(ATTR_STATS_CHANGE);

        self.ecs.insert(GameLog::new());
        self.ecs.insert(Particles::new());
        self.ecs.insert(state);
    }

    fn game_over(&mut self) {
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Delete everything failed");
        }
        self.ecs.maintain();
        self.new_campaign();
    }

    pub fn gui_tick(&self) -> Option<String> {
        let fov = self.ecs.read_storage::<FieldOfView>();
        let player = self.ecs.read_storage::<Player>();
        let player_entity = self.ecs.fetch::<PlayerEntity>();
        let positions = self.ecs.read_storage::<Position>();
        let codes = self.ecs.read_storage::<Code>();
        let inventory = self.ecs.read_storage::<InInventory>();
        let equipped = self.ecs.read_storage::<Equipped>();
        let map = self.ecs.fetch::<Map>();
        let ppos = self.ecs.fetch::<PlayerPosition>();
        let mut state = self.ecs.fetch_mut::<RunState>();
        let entities = self.ecs.entities();

        let mut hm = HashMap::new();

        if state.check_state(FOV_CHANGE) {
            let idx = map.xy_idx(ppos.position.x, ppos.position.y);
            let p = serde_json::to_value(idx).unwrap();
            let mut v = vec![p];
            let mut player_fov = Vec::new();
            for (_p, fov) in (&player, &fov).join() {
                for t in &fov.visible_tiles {
                    let idx = map.xy_idx(t.x, t.y);
                    player_fov.push(idx);
                }
            }
            let f = serde_json::to_value(player_fov).unwrap();
            v.push(f);
            hm.entry(String::from("FOV")).or_insert(serde_json::to_value(v).unwrap());
            state.remove_state(FOV_CHANGE);
        }

        if state.check_state(CONTENTS_CHANGE) {
            let mut tree: HashMap<usize, Vec<i32>> = HashMap::new();
            for (_p, fov) in (&player, &fov).join() {
                for (pos, code) in (&positions, &codes).join() {
                    if fov.visible_tiles.contains(&pos) {
                        let idx = map.xy_idx(pos.x, pos.y);
                        tree.entry(idx).or_insert(Vec::new()).push(code.code);
                    }
                }
            }
            let mut v = Vec::new();
            for (idx, mut content) in tree {
                content.sort();
                v.push((idx, content));
            }
            let contents = serde_json::to_value(v).unwrap();
            hm.entry(String::from("CONTENTS")).or_insert(contents);
            state.remove_state(CONTENTS_CHANGE);
        }

        if state.check_state(INVENTORY_CHANGE) {
            let mut pack = Vec::new();
            for (_inv, code, entity) in (&inventory, &codes, &entities)
                .join()
                .filter(|item| item.0.owner == *player_entity) {
                    pack.push((code.code, entity.id()));
                }
            let p = serde_json::to_value(pack).unwrap();
            hm.entry(String::from("INVENTORY")).or_insert(p);
            state.remove_state(INVENTORY_CHANGE);
        }

        if state.check_state(ARMOUR_CHANGE) {
            let mut body = Vec::new();
            for (_inv, code, entity) in (&equipped, &codes, &entities)
                .join()
                .filter(|item| item.0.owner == *player_entity) {
                    body.push((code.code, entity.id()));
                }
            let p = serde_json::to_value(body).unwrap();
            hm.entry(String::from("ARMOUR")).or_insert(p);
            state.remove_state(ARMOUR_CHANGE);
        }

        if state.check_state(COMBAT_STATS_CHANGE) {
            let pools = self.ecs.read_storage::<Pools>();
            let player_pools = pools.get(*player_entity).unwrap();
            let stats = vec![
                player_pools.hp.get_pool(),
                player_pools.mana.get_pool(),
            ];
            let s = serde_json::to_value(stats).unwrap();
            hm.entry(String::from("COMBAT_STATS")).or_insert(s);
            state.remove_state(COMBAT_STATS_CHANGE);
        }

        if state.check_state(ATTR_STATS_CHANGE) {
            let attributes = self.ecs.read_storage::<Attributes>();
            let attr = attributes.get(*player_entity).unwrap();
            let s = serde_json::to_value(attr.get_attributes()).unwrap();
            hm.entry(String::from("ATTR_STATS")).or_insert(s);
            state.remove_state(ATTR_STATS_CHANGE);
        }

        let mut gl = self.ecs.write_resource::<GameLog>();
        if let Some(logs) = gl.get_logs() {
            hm.entry(String::from("LOG")).or_insert(logs);
        }

        if hm.len() > 0 {
            let gm = GameMsg {
                data: json!(hm),
            };
            let s = serde_json::to_string(&gm).unwrap();
            //println!("{}", s);
            Some(s)
        } else {
            None
        }
    }

    pub fn check_particles(&self) -> Option<String> {
        let mut particles = self.ecs.write_resource::<Particles>();
        if let Some(p) = particles.get_particles() {
            let mut map = HashMap::new();
            map.entry(String::from("PARTICLES"))
                .or_insert(p);
            let gm = GameMsg {
                data: json!(map),
            };
            let s = serde_json::to_string(&gm).unwrap();
            //println!("{}", s);
            Some(s)
        } else {
            None
        }
    }

    pub fn game_tick(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.run_systems();
        if let Some(p) = self.check_particles() {
            ctx.text(p);
        }
        delete_the_dead(&mut self.ecs);
        if let Some(s) = self.gui_tick() {
            ctx.text(s);
        }
        let state;
        {
            let s = self.ecs.fetch::<RunState>();
            state = *s;
        }
        if state.check_state(GAME_OVER) {
            self.game_over();
            ctx.text(self.draw_map());
            self.run_systems();
            if let Some(s) = self.gui_tick() {
                ctx.text(s);
            }
            return;
        }
        if state.check_state(EXIT_MAP) {
            freeze_entities(&mut self.ecs);
            let (mut map, visited) = exit_map(&mut self.ecs);
            if visited {
                thaw_entities(map.key, &mut self.ecs);
            } else {
                spawn_map(&mut map, &mut self.ecs);
            }
            self.ecs.insert(map);
            ctx.text(self.draw_map());
            self.run_systems();
            if let Some(s) = self.gui_tick() {
                ctx.text(s);
            }
            return;
        }
        self.run_systems_ai();
        if let Some(p) = self.check_particles() {
            ctx.text(p);
        }
        self.run_systems();
        delete_the_dead(&mut self.ecs);
        if let Some(s) = self.gui_tick() {
            ctx.text(s);
        }
    }

    fn run_systems(&mut self) {
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut pickup_item = PickupItemSystem{};
        pickup_item.run_now(&self.ecs);
        let mut use_item = UseItemSystem{};
        use_item.run_now(&self.ecs);
        let mut drop_item = DropItemSystem{};
        drop_item.run_now(&self.ecs);
        let mut remove_item = RemoveItemSystem{};
        remove_item.run_now(&self.ecs);
        let mut trigger = TriggerSystem{};
        trigger.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        self.ecs.maintain();
    }
    
    fn run_systems_ai(&mut self) {
        let mut mob = MonsterAISystem{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }

    pub fn draw_map(&self) -> String {
        let mut state = self.ecs.fetch_mut::<RunState>();
        state.add_state(FOV_CHANGE);
        state.add_state(CONTENTS_CHANGE);
        let map = self.ecs.fetch::<Map>();
        map.draw_map()
    }
}

