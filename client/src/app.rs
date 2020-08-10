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
    pub status: Vec<String>,
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
                tiles: vec!(),
                entities: vec!(),
                status: vec!(),
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
                        let fov = get_fov_from_value(gm.data);
                        self.map.entities = vec![String::new();1200];
                        for (idx, t, e) in fov.iter() {
                            self.map.tiles[*idx] = *t;
                            self.map.entities[*idx] = (*e[0]).to_string();
                            self.map.status[*idx] = String::from("seen");
                        }
                        true
                    }
                    _ => {
                        //ConsoleService::info(&format!("{:?}", gm.data));
                        //self.map = get_map_from_value(gm.data);
                        self.map.tiles = vec![TileType::Floor;1200];
                        self.map.status = vec![String::from("not-seen");1200];
                        //ConsoleService::info(&format!("{:?}", self.map));
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

