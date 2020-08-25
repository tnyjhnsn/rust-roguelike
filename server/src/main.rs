use std::collections::{HashMap};
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

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
mod spawner;
pub use spawner::*;

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
        //println!("Tick...");

        let fov = self.ecs.read_storage::<FieldOfView>();
        let player = self.ecs.read_storage::<Player>();
        let position = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        let ppos = self.ecs.fetch::<PlayerPosition>();
        let mut state = self.ecs.fetch_mut::<RunState>();

        let mut f: Fov = Vec::new();
        let mut player_fov = Vec::new();

        for (_p, fov) in (&player, &fov).join() {
            let mut wall: Vec<usize> = Vec::new();
            let mut floor: Vec<usize> = Vec::new();
            for t in &fov.visible_tiles {
                let idx = map.xy_idx(t.x, t.y);
                match map.tiles[idx] {
                    TileType::Floor => floor.push(idx),
                    TileType::Wall => wall.push(idx),
                }
                player_fov.push(idx);
            }
            f.push((TileType::Wall, wall));
            f.push((TileType::Floor, floor));
        }

        if state.check_state(FOV_CHANGE) {
            ctx.text(send_fov(map.xy_idx(ppos.position.x, ppos.position.y), f));
            state.remove_state(FOV_CHANGE);
        }

        if state.check_state(CONTENTS_CHANGE) {
            let mut tree: HashMap<usize, Vec<String>> = HashMap::new();
            for (pos, render) in (&position, &renderable).join() {
                let idx = map.xy_idx(pos.x, pos.y);
                if player_fov.contains(&idx) {
                    tree.entry(idx).or_insert(Vec::new()).push((render.glyph).to_string());
                }
            };
            let mut e = Vec::new();
            for (idx, content) in tree {
                e.push((idx, content));
            }
            ctx.text(send_contents(e));
            state.remove_state(CONTENTS_CHANGE);
        }

        let mut gl = self.ecs.write_resource::<GameLog>();
        match gl.draw_gamelog() {
            Some(log) => ctx.text(log),
            _ => (),
        }

        //println!("...Tock");
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
                match txt.trim() {
                    "/game" => {
                        let map = self.ecs.fetch::<Map>();
                        ctx.text(map.draw_game());
                        return;
                    }
                    _ => {
                        player_input(txt, &mut self.ecs);
                        delete_the_dead(&mut self.ecs);
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
    gs.ecs.register::<Renderable>(); 
    gs.ecs.register::<Player>(); 
    gs.ecs.register::<FieldOfView>(); 
    gs.ecs.register::<Monster>(); 
    gs.ecs.register::<Name>(); 
    gs.ecs.register::<BlocksTile>(); 
    gs.ecs.register::<CombatStats>(); 
    gs.ecs.register::<SufferDamage>(); 
    gs.ecs.register::<WantsToMelee>(); 
    gs.ecs.register::<Item>(); 
    gs.ecs.register::<Potion>(); 

    let mut map = Map::new();
    map.create_temp_walls();
    let px = 20;
    let py = 20;

    let player = player(&mut gs.ecs, px, py);
    gs.ecs.insert(player);

    for i in 1..8 {
        let (x, y) = map.get_random_space();
        random_monster(&mut gs.ecs, x, y, i);
    }

    for _i in 1..8 {
        let (x, y) = map.get_random_space();
        health_potion(&mut gs.ecs, x, y);
    }

    gs.ecs.insert(PlayerPosition::new(px, py));
    gs.ecs.insert(map);
    gs.ecs.insert(GameLog::new());
    gs.ecs.insert(RunState::new(WAITING));
    
    let res = ws::start(gs, &req, stream);
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
