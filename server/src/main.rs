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
mod spawner;
pub use spawner::*;
mod dungeon;
pub use dungeon::*;

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

struct GameSocket {
    ecs: World,
}

impl GameSocket {
    fn tick(&mut self,  ctx: &mut ws::WebsocketContext<Self>) {

        let fov = self.ecs.read_storage::<FieldOfView>();
        let player = self.ecs.read_storage::<Player>();
        let player_entity = self.ecs.fetch::<Entity>();
        let positions = self.ecs.read_storage::<Position>();
        let codes = self.ecs.read_storage::<Code>();
        let inventory = self.ecs.read_storage::<InInventory>();
        let map = self.ecs.fetch::<Map>();
        let ppos = self.ecs.fetch::<PlayerPosition>();
        let mut state = self.ecs.fetch_mut::<RunState>();
        let entities = self.ecs.entities();

        let mut fov_tiles = Vec::new();
        let mut player_fov = Vec::new();

        for (_p, fov) in (&player, &fov).join() {
            let mut wall: Vec<usize> = Vec::new();
            let mut floor: Vec<usize> = Vec::new();
            let mut stairs: Vec<usize> = Vec::new();
            for t in &fov.visible_tiles {
                let idx = map.xy_idx(t.x, t.y);
                match map.tiles[idx] {
                    TileType::Floor => floor.push(idx),
                    TileType::Wall => wall.push(idx),
                    TileType::DownStairs => stairs.push(idx),
                }
                player_fov.push(idx);
            }
            fov_tiles.push((TileType::Wall, wall));
            fov_tiles.push((TileType::Floor, floor));
            fov_tiles.push((TileType::DownStairs, stairs));
        }

        let mut hm = HashMap::new();

        if state.check_state(FOV_CHANGE) {
            let idx = map.xy_idx(ppos.position.x, ppos.position.y);
            let p = serde_json::to_value(idx).unwrap();
            let f = serde_json::to_value(fov_tiles).unwrap();
            let mut v = Vec::new();
            v.push(p);
            v.push(f);
            hm.entry(String::from("FOV")).or_insert(serde_json::to_value(v).unwrap());
            state.remove_state(FOV_CHANGE);
        }

        if state.check_state(CONTENTS_CHANGE) {
            let mut tree: HashMap<usize, Vec<i32>> = HashMap::new();
            for (pos, code) in (&positions, &codes).join() {
                let idx = map.xy_idx(pos.x, pos.y);
                if player_fov.contains(&idx) {
                    tree.entry(idx).or_insert(Vec::new()).push(code.code);
                }
            };
            let mut v = Vec::new();
            for (idx, content) in tree {
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

        if hm.len() > 0 {
            let gm = GameMsg {
                data: json!(hm),
            };
            let s = serde_json::to_string(&gm).unwrap();
            //println!("{}", s);
            ctx.text(s);
        }

        let mut gl = self.ecs.write_resource::<GameLog>();
        match gl.draw_gamelog() {
            Some(log) => ctx.text(log),
            _ => (),
        }
    }

    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
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
        self.ecs.maintain();
    }
    
    fn run_systems_ai(&mut self) {
        let mut mob = MonsterAISystem{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
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
                            "/game" => {
                                get_game(&mut self.ecs, ctx);
                            }
                            "g"|"G" => {
                                get_item(&mut self.ecs);
                            }
                            ">" => {
                                get_downstairs(&mut self.ecs);
                                get_game(&mut self.ecs, ctx);
                            }
                            _ => {
                                player_input(txt, &mut self.ecs);
                                delete_the_dead(&mut self.ecs);
                            }
                        }
                        self.run_systems();
                        self.tick(ctx);
                    }
                    _ => {
                        let idx = chunks[1].parse::<u64>().unwrap();
                        match chunks[0] {
                            "/drop" => {
                                drop_item(idx, &mut self.ecs);
                            }
                            "/use" => {
                                let t = chunks[2].parse::<i32>().unwrap();
                                let target = if t == -1 { None } else { Some(t) };
                                use_item(idx, target, &mut self.ecs);
                            }
                            _ => ()
                        }
                        self.run_systems();
                        self.tick(ctx);
                    }
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
        self.tick(ctx);
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
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

    let px = 20;
    let py = 20;
    let player = player(&mut gs.ecs, px, py);
    gs.ecs.insert(player);
    gs.ecs.insert(PlayerPosition::new(px, py));

    let mut map = Map::new(0);
    map.spawn_map(&mut gs.ecs);
    gs.ecs.insert(map);
    
    gs.ecs.insert(GameLog::new());
    gs.ecs.insert(RunState::new(WAITING));
    gs.ecs.insert(Dungeon::new());

    let res = ws::start(gs, &req, stream);
    println!("{:?}", res);
    res
}

fn get_game(ecs: &mut World, ctx: &mut ws::WebsocketContext<GameSocket>) {
    let map = ecs.fetch::<Map>();
    ctx.text(map.draw_game());
    let mut state = ecs.fetch_mut::<RunState>();
    state.add_state(FOV_CHANGE);
    state.add_state(CONTENTS_CHANGE);
}

fn get_downstairs(ecs: &mut World) {
    if !try_next_level(ecs) { return; };
    let to_delete = entities_to_remove_on_level_change(ecs);
    for target in to_delete {
        ecs.delete_entity(target).expect("Unable to delete entity");
    }
    let mut new_map = down_stairs(ecs);
    new_map.spawn_map(ecs);
    ecs.insert(new_map);
}

fn try_next_level(ecs: &mut World) -> bool {
    let ppos = ecs.fetch::<PlayerPosition>();
    let map = ecs.fetch::<Map>();
    let ppos_idx = map.xy_idx(ppos.position.x, ppos.position.y);

    if map.tiles[ppos_idx] == TileType::DownStairs {
        true
    } else {
        let mut gamelog = ecs.fetch_mut::<GameLog>();
        gamelog.add_log(vec![LogType::System as i32, 2]);
        false
    }
}

fn entities_to_remove_on_level_change(ecs: &mut World) -> Vec<Entity> {
    let entities = ecs.entities();
    let player = ecs.read_storage::<Player>();
    let inventory = ecs.read_storage::<InInventory>();
    let player_entity = ecs.fetch::<Entity>();

    let mut to_delete : Vec<Entity> = Vec::new();
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
        if should_delete { 
            to_delete.push(entity);
        }
    }
    to_delete
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
