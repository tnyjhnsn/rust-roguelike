use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use serde::{Serialize};
use serde_json::Value;

use specs::prelude::*;
use roguelike_common::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;

struct GameSocket {
    ecs: World
}

impl GameSocket {
    fn tick(&mut self,  txt: String, ctx: &mut ws::WebsocketContext<Self>) {
        println!("Tick...");

        player_input(txt, &mut self.ecs);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, _render) in (&positions, &renderables).join() {
            //println!("{} {} {}", pos.x, pos.y, render.glyph);
            ctx.text(get_position(pos.x, pos.y));
        }

        println!("...Tock");
    }

    fn run_systems(&mut self) {
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
                        let tiles = self.ecs.fetch::<Vec<TileType>>();
                        ctx.text(draw_map((&tiles).to_vec()));
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

    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 20, y: 10 })
        .with(Renderable { glyph: '@' })
        .with(Player{})
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
