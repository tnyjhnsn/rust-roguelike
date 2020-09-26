use actix::{
    Actor,
    StreamHandler,
};
use actix_web::{
    web,
    App,
    Error,
    HttpRequest,
    HttpResponse,
    HttpServer,
};
use actix_web_actors::ws;

use specs::prelude::*;
use roguelike_common::*;

mod maps;
pub use maps::*;

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
mod random_table;
pub use random_table::*;
mod level_change;
pub use level_change::*;
mod campaign;
pub use campaign::*;
mod tick;
pub use tick::*;

pub struct GameSocket {
    ecs: World,
    campaign: Campaign,
}

impl GameSocket {

    fn new() -> Self {
        let mut gs = GameSocket {
            ecs: World::new(),
            campaign: Campaign::new(),
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
        gs.ecs.register::<EntityMoved>(); 
        gs.new_game();
        gs
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
                            _ => {
                                player_input(txt, &mut self.ecs);
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
                            }
                            _ => ()
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Bin {:?}", bin);
                ctx.binary(bin);
            }
            _ => (),
        }
        self.game_tick(ctx)
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
