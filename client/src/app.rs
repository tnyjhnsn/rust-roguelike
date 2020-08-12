use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::ConsoleService;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use roguelike_common::*;

use super::dungeon::*;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub entities: Vec<String>,
    pub status: Vec<i32>,
    pub current_fov: Vec<usize>,
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
    GetMap,
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
                tiles: vec![TileType::Floor; 1200],
                entities: vec![String::new(); 1200],
                status: vec![0; 1200],
                current_fov: Vec::new(),
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
    		Msg::GetMap => {
    			match self.ws {
    				Some(ref mut task) => {
    					task.send(Ok("/map".to_string()));
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
                    "FOV" => {
                        for c in &self.map.current_fov {
                            self.map.status[*c] &= !VISIBLE;
                            self.map.status[*c] |= SEEN;
                        }
                        self.map.current_fov.clear();
                        let fov = get_fov_from_value(gm.data);
                        for (idx, tile) in fov.iter() {
                            self.map.tiles[*idx] = *tile;
                            self.map.status[*idx] |= VISIBLE;
                            self.map.current_fov.push(*idx);
                        }
                        true
                    }
                    "ENTITIES" => {
                        self.map.entities = vec![String::new(); 1200];
                        let entities = get_entities_from_value(gm.data);
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
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Ok(e.key()));
                        false
                    }
                    None => false
                }
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
                <p><button onclick=self.link.callback(|_| Msg::GetMap)>{ "Get Map" }</button></p>
                <Dungeon map=&self.map />
            </>
    	}
    }
}

