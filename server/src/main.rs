use std::collections::{HashMap};
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde_json::json;

use specs::prelude::*;
use roguelike_common::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod visibility_system;
pub use visibility_system::*;
mod monster_ai_system;
pub use monster_ai_system::*;
mod gamelog;
pub use gamelog::*;
mod map_indexing_system;
pub use map_indexing_system::*;
mod melee_combat_system;
pub use melee_combat_system::*;
mod damage_system;
pub use damage_system::*;
mod inventory_system;
pub use inventory_system::*;
mod trigger_system;
pub use trigger_system::*;
mod spawner;
pub use spawner::*;
mod dungeon;
pub use dungeon::*;
mod random_table;
pub use random_table::*;
mod level_change;
pub use level_change::*;
mod dwarven_mines_gate;
pub use dwarven_mines_gate::*;
mod dwarven_mines_hall;
pub use dwarven_mines_hall::*;

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

pub struct GameSocket {
    ecs: World,
}

impl GameSocket {

    fn new() -> Self {
        let mut gs = GameSocket {
            ecs: World::new(),
        };
        gs.ecs.register::<Position>(); 
        gs.ecs.register::<Code>(); 
        gs.ecs.register::<Player>(); 
        gs.ecs.register::<FieldOfView>(); 
        gs.ecs.register::<Monster>(); 
        gs.ecs.register::<Code>(); 
        gs.ecs.register::<BlocksTile>(); 
        gs.ecs.register::<CombatStats>(); 
        gs.ecs.register::<HealthStats>(); 
        gs.ecs.register::<SufferDamage>(); 
        gs.ecs.register::<WantsToMelee>(); 
        gs.ecs.register::<Item>(); 
        gs.ecs.register::<Consumeable>(); 
        gs.ecs.register::<Ranged>(); 
        gs.ecs.register::<AreaOfEffect>(); 
        gs.ecs.register::<ProvidesHealing>(); 
        gs.ecs.register::<Confusion>(); 
        gs.ecs.register::<InflictsDamage>(); 
        gs.ecs.register::<InInventory>(); 
        gs.ecs.register::<WantsToPickupItem>(); 
        gs.ecs.register::<WantsToDropItem>(); 
        gs.ecs.register::<WantsToUseItem>(); 
        gs.ecs.register::<WantsToRemoveItem>(); 
        gs.ecs.register::<Equippable>(); 
        gs.ecs.register::<Equipped>(); 
        gs.ecs.register::<MeleePowerBonus>(); 
        gs.ecs.register::<DefenseBonus>(); 
        gs.ecs.register::<EntryTrigger>(); 
        gs.new_game();
        gs
    }

    fn new_game(&mut self) {
        let px = 15;
        let py = 58;
        let player = player(&mut self.ecs, px, py);
        self.ecs.insert(player);
        self.ecs.insert(PlayerPosition::new(px, py));

        let mut map = Map::new(0);
        spawn_map(&mut map, &mut self.ecs);
        self.ecs.insert(map);
        
        self.ecs.insert(GameLog::new());
        self.ecs.insert(RunState::new(WAITING));
        self.ecs.insert(Dungeon::new());
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
    }

    fn tick(&self) -> Option<String> {
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
                    if fov.visible_tiles.contains(&pos.to_point()) {
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

    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
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

    fn draw_map(&self) -> String {
        let map = self.ecs.fetch::<Map>();
        let mut state = self.ecs.fetch_mut::<RunState>();
        state.add_state(FOV_CHANGE);
        state.add_state(CONTENTS_CHANGE);
        map.draw_map()
    }
}

impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        //println!("MSG {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(m)) => ctx.pong(&m),
            Ok(ws::Message::Text(txt)) => {
                let chunks: Vec<&str> = txt.trim().split(" ").collect();
                match chunks.len() {
                    1 => {
                        match chunks[0] {
                            "/map" => {
                                ctx.text(self.draw_map());
                            }
                            "g"|"G" => {
                                pickup_item(&mut self.ecs);
                            }
                            ">" => {
                                if try_next_level(&mut self.ecs) {
                                    self.go_downstairs();
                                    ctx.text(self.draw_map());
                                }
                            }
                            _ => {
                                player_input(txt, &mut self.ecs);
                                delete_the_dead(&mut self.ecs);
                            }
                        }
                    }
                    _ => {
                        let idx = chunks[1].parse::<u64>().unwrap();
                        match chunks[0] {
                            "/drop" => {
                                drop_item(idx, &mut self.ecs);
                            }
                            "/remove" => {
                                remove_item(idx, &mut self.ecs);
                            }
                            "/use" => {
                                let t = chunks[2].parse::<i32>().unwrap();
                                let target = if t == -1 { None } else { Some(t) };
                                use_item(idx, target, &mut self.ecs);
                                delete_the_dead(&mut self.ecs);
                            }
                            _ => ()
                        }
                    }
                }
                self.run_systems();
                if let Some(s) = self.tick() {
                    ctx.text(s);
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Bin {:?}", bin);
                ctx.binary(bin);
            }
            _ => (),
        }
        self.run_systems_ai();
        delete_the_dead(&mut self.ecs);
        self.run_systems();
        if let Some(s) = self.tick() {
            ctx.text(s);
        }
        let state;
        {
            let s = self.ecs.fetch::<RunState>();
            state = *s;
        }
        if state.check_state(GAME_OVER) {
            self.game_over();
            self.new_game();
            ctx.text(self.draw_map());
            self.run_systems();
            if let Some(s) = self.tick() {
                ctx.text(s);
            }
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(GameSocket::new(), &req, stream);
    println!("{:?}", res);
    res
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(index)))
            // static files
            //.service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:9001
    .bind("127.0.0.1:9001")?
    .run()
    .await
}
