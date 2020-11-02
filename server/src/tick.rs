use std::collections::{HashMap};
use super::*;
use crate::ai::{
    InitiativeSystem,
    EncumbranceSystem,
    TurnStatusSystem,
    VisibleAI,
    AdjacentAI,
    ApproachAI,
    FleeAI,
    DefaultMoveAI,
};

use actix::{
    Actor,
};
use serde_json::json;

#[derive(PartialEq, Copy, Clone)]
pub struct GuiState {
    state: i32,
}

impl GuiState {
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameState {
    Waiting,
    Ticking,
    ExitMap,
    GameOver,
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

        let mut gui_state = GuiState::new(WAITING);
        gui_state.add_state(INVENTORY_CHANGE);
        gui_state.add_state(ARMOUR_CHANGE);
        gui_state.add_state(COMBAT_STATS_CHANGE);
        gui_state.add_state(ATTR_STATS_CHANGE);
        gui_state.add_state(XP_CHANGE);
        self.ecs.insert(gui_state);

        self.ecs.insert(GameState::Waiting);
        self.ecs.insert(GameLog::new());
        self.ecs.insert(Particles::new());
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
        let mut state = self.ecs.fetch_mut::<GuiState>();
        let entities = self.ecs.entities();

        let mut hm = HashMap::new();

        if state.check_state(FOV_CHANGE) {
            for (_, fov) in (&player, &fov).join() {
                let mut v = vec![json!(fov.range)];
                let idx = map.xy_idx(ppos.position.x, ppos.position.y);
                v.push(json!(idx));
                let mut player_fov = Vec::new();
                for t in &fov.visible_tiles {
                    let idx = map.xy_idx(t.x, t.y);
                    player_fov.push(idx);
                }
                v.push(json!(player_fov));
                hm.entry(String::from("FOV")).or_insert(json!(v));
                state.remove_state(FOV_CHANGE);
            }
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
            hm.entry(String::from("CONTENTS")).or_insert(json!(v));
            state.remove_state(CONTENTS_CHANGE);
        }

        if state.check_state(INVENTORY_CHANGE) {
            let mut pack = Vec::new();
            for (_inv, code, entity) in (&inventory, &codes, &entities)
                .join()
                .filter(|item| item.0.owner == *player_entity) {
                    pack.push((code.code, entity.id()));
                }
            hm.entry(String::from("INVENTORY")).or_insert(json!(pack));
            let pools = self.ecs.read_storage::<Pools>();
            let player_pools = pools.get(*player_entity).unwrap();
            let weight = player_pools.tot_weight;
            let capacity = player_pools.carry_capacity;
            let penalty = player_pools.tot_initiative_penalty;
            let encumbrance = vec![weight, capacity, penalty];
            hm.entry(String::from("ENCUMBRANCE")).or_insert(json!(encumbrance));
            state.remove_state(INVENTORY_CHANGE);
        }

        if state.check_state(ARMOUR_CHANGE) {
            let mut body = Vec::new();
            for (_inv, code, entity) in (&equipped, &codes, &entities)
                .join()
                .filter(|item| item.0.owner == *player_entity) {
                    body.push((code.code, entity.id()));
                }
            hm.entry(String::from("ARMOUR")).or_insert(json!(body));
            state.remove_state(ARMOUR_CHANGE);
        }

        if state.check_state(COMBAT_STATS_CHANGE) {
            let pools = self.ecs.read_storage::<Pools>();
            let player_pools = pools.get(*player_entity).unwrap();
            let stats = vec![
                player_pools.hp.get_pool(),
                player_pools.mana.get_pool(),
            ];
            hm.entry(String::from("COMBAT_STATS")).or_insert(json!(stats));
            state.remove_state(COMBAT_STATS_CHANGE);
        }

        if state.check_state(ATTR_STATS_CHANGE) {
            let attributes = self.ecs.read_storage::<Attributes>();
            let attr = attributes.get(*player_entity).unwrap();
            let a = attr.get_attributes();
            hm.entry(String::from("ATTR_STATS")).or_insert(json!(a));
            state.remove_state(ATTR_STATS_CHANGE);
        }

        if state.check_state(XP_CHANGE) {
            let pools = self.ecs.read_storage::<Pools>();
            let player_pools = pools.get(*player_entity).unwrap();
            let stats = (player_pools.level, player_pools.xp);
            hm.entry(String::from("LEVEL_STATS")).or_insert(json!(stats));
            state.remove_state(XP_CHANGE);
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
        let mut game_state;
        {
            let g = self.ecs.fetch::<GameState>();
            game_state = *g;
        }

        match game_state {
            GameState::ExitMap => {
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
                game_state = GameState::Ticking;
            }
            GameState::Ticking => {
                while game_state == GameState::Ticking {
                    self.run_systems();
                    if let Some(p) = self.check_particles() {
                        ctx.text(p);
                    }
                    delete_the_dead(&mut self.ecs);
                    if let Some(s) = self.gui_tick() {
                        ctx.text(s);
                    }
                    game_state = *self.ecs.fetch::<GameState>();
                }
            }
            _ => {}
        }

        if game_state == GameState::GameOver {
            self.game_over();
            ctx.text(self.draw_map());
            self.run_systems();
            if let Some(s) = self.gui_tick() {
                ctx.text(s);
            }
            game_state = GameState::Ticking;
        }

        {
            let mut g = self.ecs.write_resource::<GameState>();
            *g = game_state;
        }
    }

    fn run_systems(&mut self) {
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut initiative = InitiativeSystem{};
        initiative.run_now(&self.ecs);
        let mut turn_status = TurnStatusSystem{};
        turn_status.run_now(&self.ecs);
        let mut adjacent = AdjacentAI{};
        adjacent.run_now(&self.ecs);
        let mut visible = VisibleAI{};
        visible.run_now(&self.ecs);
        let mut approach = ApproachAI{};
        approach.run_now(&self.ecs);
        let mut flee = FleeAI{};
        flee.run_now(&self.ecs);
        let mut defaultmove = DefaultMoveAI{};
        defaultmove.run_now(&self.ecs);
        let mut trigger = TriggerSystem{};
        trigger.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem{};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem{};
        damage.run_now(&self.ecs);
        let mut pickup_item = PickupItemSystem{};
        pickup_item.run_now(&self.ecs);
        let mut use_item = UseItemSystem{};
        use_item.run_now(&self.ecs);
        let mut drop_item = DropItemSystem{};
        drop_item.run_now(&self.ecs);
        let mut remove_item = RemoveItemSystem{};
        remove_item.run_now(&self.ecs);

        self.ecs.maintain();

        let mut encumbrance = EncumbranceSystem{};
        encumbrance.run_now(&self.ecs);
    }
    
    pub fn draw_map(&self) -> String {
        let mut state = self.ecs.fetch_mut::<GuiState>();
        let mut game_state = self.ecs.write_resource::<GameState>();
        state.add_state(FOV_CHANGE);
        state.add_state(CONTENTS_CHANGE);
        *game_state = GameState::Ticking;
        let map = self.ecs.fetch::<Map>();
        map.draw_map()
    }
}
