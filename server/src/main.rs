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

struct GameSocket {
    ecs: World
}

impl GameSocket {
    fn tick(&mut self,  txt: String, ctx: &mut ws::WebsocketContext<Self>) {
        println!("Tick...");

        player_input(txt, &mut self.ecs);
        self.run_systems();

        let fovs = self.ecs.read_storage::<FieldOfView>();
        let player = self.ecs.read_storage::<Player>();
        let position = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        let mut f: Fov = vec!();

        for (_p, pos, fov, render) in (&player, &position, &fovs, &renderable).join() {
            for t in &fov.visible_tiles {
                let idx = map.xy_idx(t.x, t.y);
                let s = if (pos.x, pos.y) == (t.x, t.y) { (render.glyph).to_string() } else { String::new() };
                f.push((idx, map.tiles[idx], vec![s]));
            }
        }

        ctx.text(draw_fov(f));
        println!("...Tock");
    }

    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
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
        println!("MSG {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(m)) => ctx.pong(&m),
            Ok(ws::Message::Text(txt)) => {
                match txt.trim() {
                    "/map" => {
                        let map = self.ecs.fetch::<Map>();
                        ctx.text(map.draw_map());
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

    let mut map = Map::new_map();
    map.create_temp_walls();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: 20, y: 10 })
        .with(Renderable { glyph: String::from("player-m") })
        .with(Player{})
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: 5,
            dirty: true,
        })
        .build();

    
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
