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

struct GameSocket {
    ecs: World
}

impl GameSocket {
    fn tick(&mut self,  txt: String, ctx: &mut ws::WebsocketContext<Self>) {
        //println!("Tick...");

        player_input(txt, &mut self.ecs);
        self.run_systems();

        let fovs = self.ecs.read_storage::<FieldOfView>();
        let player = self.ecs.read_storage::<Player>();
        let position = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        let mut f: Fov = Vec::new();
        let mut player_fov = Vec::new();

        for (_p, fov) in (&player, &fovs).join() {
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

        let mut e: roguelike_common::Entities = Vec::new();

        for (pos, render) in (&position, &renderable).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if player_fov.contains(&idx) {
                e.push((idx, vec![(render.glyph).to_string()]));
            }
        };

        ctx.text(draw_fov(f, e));

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
        let mut mob = MonsterAISystem{};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
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
                    }
                    _ => {
                        self.tick(txt, ctx);
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Bin {:?}", bin);
                ctx.binary(bin);
            }
            _ => (),
        }
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

    let mut map = Map::new_map();
    map.create_temp_walls();
    let px = 20;
    let py = 20;

    gs.ecs
        .create_entity()
        .with(Position { x: px, y: py })
        .with(Renderable { glyph: String::from("player-m") })
        .with(Player{})
        .with(Name { name: "The Hero".to_string() })
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
        })
        .build();

    let mut rng = rltk::RandomNumberGenerator::new();

    for i in 1..8 {
        let (x, y) = map.get_random_space();
        let glyph;
        let name;
        let roll = rng.roll_dice(1, 5);
        match roll {
            1 => { glyph = String:: from("white-centipede"); name = "Carnivorous White Centipede".to_string(); }
            2 => { glyph = String:: from("red-ant"); name = "Giant Red Ant".to_string(); }
            3 => { glyph = String:: from("ghost"); name = "Scary Ghost".to_string(); }
            _ => { glyph = String:: from("grey-mould"); name = "Grey Mould".to_string(); }
        }
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable { glyph })
            .with(Monster{})
            .with(Name { name: format!("{} #{}", &name, i) })
            .with(FieldOfView {
                visible_tiles: Vec::new(),
                range: 5,
            })
            .with(BlocksTile{})
            .build();
    }

    gs.ecs.insert(
        PlayerPosition {
            position: Point::new(px, py),
            dijkstra_map: Vec::new(),
        });
    gs.ecs.insert(map);

    gs.ecs.insert(GameLog::new());
    
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
