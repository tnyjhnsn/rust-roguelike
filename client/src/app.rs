use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use roguelike_common::*;

pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    key_listener: KeyListenerHandle,
    game: Game,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    GetMap,
    Received(Result<Value, Error>),
    Pressed(KeyboardEvent),
}

#[derive(Serialize)]
struct WsRequest {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GameMsg {
    msg: String,
    data: Value,
}

fn get_gamemsg_from_value(v: Value) -> GameMsg {
    serde_json::from_value(v).unwrap()
}

fn get_position_from_value(v: Value) -> Position {
    serde_json::from_value(v).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
struct Game {
    width: i32,
    height: i32,
    tiles: Vec<TileType>,
}

fn get_game_from_value(v: Value) -> Game {
    serde_json::from_value(v).unwrap()
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window = &web_sys::window().unwrap();
        let key_listener = KeyboardService::register_key_down(&window,
            (&link).callback(|e: KeyboardEvent| {e.prevent_default(); Msg::Pressed(e)}));
    	Model {
    		ws: None,
    		link: link,
            key_listener,
            game: Game { width: 0, height: 0, tiles: vec!() }
    	}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
    	match msg {
    		Msg::Connect => {
    			let cbout = self.link.callback(|Json(data)| Msg::Received(data));
    			let cbnot = self.link.callback(|input| {
    				match input {
    					WebSocketStatus::Closed | WebSocketStatus::Error => {
    						Msg::Disconnected
    					}
    					_ => Msg::Ignore,
    				}
    			});
    			if self.ws.is_none() {
    				let task = WebSocketService::connect("ws://127.0.0.1:9001/ws/",
                                                         cbout, cbnot.into());
    				self.ws = Some(task.unwrap());
    			}
    			true
    		}
    		Msg::Disconnected => {
    			self.ws = None;
    			true
    		}
    		Msg::Ignore => {
    			false
    		}
    		Msg::GetMap => {
    			match self.ws {
    				Some(ref mut task) => {
    					task.send(Ok("/map".to_string()));
    					true
    				}
    				None => {
    					false
    				}
    			}
    		}
    		Msg::Received(Ok(v)) => {
                let gm: GameMsg = get_gamemsg_from_value(v);
                match gm.msg.trim() {
                    "POSITION" => {
                        ConsoleService::info(&format!("{:?}", get_position_from_value(gm.data)));
                        true
                    }
                    _ => {
                        self.game = get_game_from_value(gm.data);
                        //ConsoleService::info(&format!("{:?}", self.game));
                        true
                    }
                }
    		}
    		Msg::Received(Err(s)) => {
    			let msg = format!("Error when reading data from server: {}\n", &s.to_string());
                ConsoleService::info(&format!("{}", msg));
    			false
    		}
            Msg::Pressed(e) => {
    			match self.ws {
    				Some(ref mut task) => {
    					task.send(Ok(e.key()));
                        false
    				}
    				None => {
    					false
    				}
    			}
            }
    	}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        ConsoleService::info("Change called");
        true
    }

    fn view(&self) -> Html {
    	html! {
            <>
                <h1 class="title">{ "Rogue" }</h1>
                    <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button><br/>
                    { "Connected: " } { !self.ws.is_none() }
                    <p><button onclick=self.link.callback(|_| Msg::GetMap)>{ "Get Map" }</button></p>
                <div class="dungeon">
                    <div class="level">
                        { for self.game.tiles.iter().map(display_tile) }
                    </div>
                </div>
            </>
    	}
    }
}

fn display_tile(tile: &TileType) -> Html {
    let tile_status = if *tile == TileType::Wall { "wall" } else { "floor" };
    html! {
        <div class=("tile", tile_status)>
            <div class="tile dagger"></div>
        </div>
    }
}

