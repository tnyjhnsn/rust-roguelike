use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
//use actix_files as fs;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use serde_repr::*;

use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

#[derive(Component, Debug, Serialize, Deserialize)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: char,
}

#[derive(Component, Debug)]
struct Player {}

#[derive(Debug, PartialEq, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum TileType {
    Wall = 0,
    Floor = 1,
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[dest_idx] != TileType::Wall {
            pos.x = min(59 , max(0, pos.x + delta_x));
            pos.y = min(19, max(0, pos.y + delta_y));
        }
    }
}

fn player_input( txt: String, ecs: &mut World) {
    match txt.trim() {
        "ArrowLeft" => {
            try_move_player(-1, 0, ecs);
        }
        "ArrowRight" => {
            try_move_player(1, 0, ecs);
        }
        "ArrowUp" => {
            try_move_player(0, -1, ecs);
        }
        "ArrowDown" => {
            try_move_player(0, 1, ecs);
        }
        _ => ()
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 60) + x as usize
}

fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 60*20];

    for x in 0..60 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 19)] = TileType::Wall;
    }
    for y in 0..20 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(59, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..200 {
        let x = rng.roll_dice(1, 59);
        let y = rng.roll_dice(1, 19);
        let idx = xy_idx(x, y);
        if idx != xy_idx(20, 10) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

#[derive(Debug, Serialize)]
struct Action {
    msg: String,
    data: Value,
}

fn get_position(x: i32, y: i32) -> String {
    let p = Position { x, y };
    let action = Action {
        msg: String::from("POSITION"),
        data: serde_json::to_value(p).unwrap(),
    };
    println!("getting position");
    serde_json::to_string(&action).unwrap()
}

#[derive(Serialize)]
struct Game {
    width: i32,
    height: i32,
    tiles: Vec<TileType>
}

fn draw_map(tiles: Vec<TileType>) -> String {
    //println!("{}", serde_json::to_string(&tiles).unwrap());
    //let nums = vec![TileType::Wall, TileType::Floor];
    //println!("{}", serde_json::to_string(&nums).unwrap());

    let g = Game { 
        width: 60,
        height: 20,
        tiles,
    };
    let action = Action {
        msg: String::from("GAME"),
        data: serde_json::to_value(g).unwrap(),
    };
    let map = serde_json::to_string(&action).unwrap();
    println!("{}", map);
    map
}

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
