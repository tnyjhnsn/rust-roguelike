use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::ConsoleService;
use serde::{Serialize};
use serde_json::Value;
use roguelike_common::*;

use super::dungeon::*;
use super::viewport::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub entities: Vec<String>,
    pub status: Vec<i32>,
    pub current_fov: Vec<usize>,
    pub viewport: Viewport,
}


pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    key_listener: KeyListenerHandle,
    map: Map,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    GetGame,
    Received(Result<Value, Error>),
    Pressed(KeyboardEvent),
}

#[derive(Serialize)]
struct WsRequest {
    value: String,
}

fn get_gamemsg_from_value(v: Value) -> GameMsg {
    serde_json::from_value(v).unwrap()
}

fn get_game_from_value(v: Value) -> (i32, i32) {
    serde_json::from_value(v).unwrap()
}

fn get_fov_from_value(v: Value) -> Fov {
    serde_json::from_value(v).unwrap()
}

fn get_entities_from_value(v: Value) -> Entities {
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
            map: Map {
                width: 0,
                height: 0,
                tiles: Vec::new(),
                entities: Vec::new(),
                status: Vec::new(),
                current_fov: Vec::new(),
                viewport: Viewport::new(0),
            },
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
    			false
    		}
    		Msg::Disconnected => {
    			self.ws = None;
    			false
    		}
    		Msg::Ignore => {
    			false
    		}
    		Msg::GetGame => {
    			match self.ws {
    				Some(ref mut task) => {
    					task.send(Ok("/game".to_string()));
    					false
    				}
    				None => {
    					false
    				}
    			}
    		}
    		Msg::Received(Ok(v)) => {
                let gm: GameMsg = get_gamemsg_from_value(v);
                match gm.msg.trim() {
                    "GAME" => {
                        let game = get_game_from_value(gm.data);
                        let width = game.0;
                        let height = game.1;
                        let dim = (width * height) as usize;
                        self.map = Map {
                            width,
                            height,
                            tiles: vec![TileType::Floor; dim],
                            entities: vec![String::new(); dim],
                            status: vec![0; dim],
                            current_fov: Vec::new(),
                            viewport:Viewport::new(width),
                        };
                        true
                    }
                    "FOV" => {
                        for c in &self.map.current_fov {
                            self.map.status[*c] &= !VISIBLE;
                            self.map.status[*c] |= SEEN;
                        }
                        self.map.current_fov.clear();
                        let data = &gm.data;
                        let fov = get_fov_from_value(data[0].clone());
                        let entities = get_entities_from_value(data[1].clone());
                        let ppos = entities[0].0;
                        self.map.viewport.set_indexes(ppos as i32);
                        for (tile, indexes) in fov.iter() {
                            for idx in indexes.iter() {
                                self.map.tiles[*idx] = *tile;
                                self.map.status[*idx] |= VISIBLE;
                                self.map.current_fov.push(*idx);
                            }
                        }
                        let w = self.map.width;
                        let h = self.map.height;
                        let dim = (w * h) as usize;
                        self.map.entities = vec![String::new(); dim];
                        for (idx, entity) in entities.iter() {
                            self.map.entities[*idx] = (*entity[0]).to_string();
                        }
                        true
                    }
                    _ => {
                        //ConsoleService::info(&format!("{:?}", gm.data));
                        false
                    }
                }
    		}
            Msg::Received(Err(s)) => {
                let msg = format!("Error when reading data from server: {}\n", &s.to_string());
                ConsoleService::info(&format!("{}", msg));
                false
            }
            Msg::Pressed(e) => {
                if e.key_code() >= 37 && e.key_code() <= 90 {
                    match self.ws {
                        Some(ref mut task) => {
                            task.send(Ok(e.key()));
                            false
                        }
                        None => false
                    }
                } else { false }
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
    	html! {
            <>
                <h1 class="title">{ "Rogue" }</h1>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button><br/>
                { "Connected: " } { !self.ws.is_none() }
                <p><button onclick=self.link.callback(|_| Msg::GetGame)>{ "Get Game Dimensions" }</button></p>
                <Dungeon map=&self.map />
            </>
    	}
    }
}

