use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::ConsoleService;
use serde::{Serialize};
use serde_json::Value;
use roguelike_common::*;
use std::collections::HashMap;
use web_sys::{HtmlElement};
use yew::utils::document;
use wasm_bindgen::JsCast;

use super::model::game_model::*;
use super::game::*;

pub struct Targeter {
    pub fov: Vec<usize>,
    pub idx: i32,
    pub width: i32,
}

impl Targeter {
    // TODO idx is current ppos from MMap
    pub fn new() -> Self {
        Self {
            fov: Vec::new(),
            idx: 0,
            width: 0,
        }
    }

    pub fn create(&mut self, fov: &Vec<usize>, idx: i32, width: i32) {
        self.fov = fov.to_vec();
        self.idx = idx;
        self.width = width;
    }

    fn move_target(&mut self, x: i32, y: i32) -> Option<i32> {
        let pos = self.idx_xy(self.idx);
        let new_pos = self.xy_idx(pos.x + x, pos.y + y);
        let p = new_pos as usize;
        if self.fov.contains(&p) {
            self.idx = new_pos;
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn try_move(&mut self, e: KeyboardEvent) -> Option<i32> {
        match e.key_code() {
            KEY_LEFT => self.move_target(-1, 0),
            KEY_RIGHT => self.move_target(1, 0),
            KEY_UP => self.move_target(0, -1),
            KEY_DOWN => self.move_target(0, 1),
            KEY_Y => self.move_target(-1, -1),
            KEY_U => self.move_target(1, -1),
            KEY_N => self.move_target(1, 1),
            KEY_B => self.move_target(-1, 1),
            _ => None
        }
    }

    fn xy_idx(&self, x: i32, y: i32) -> i32 {
        (y * self.width) + x
    }

    fn idx_xy(&self, idx: i32) -> Point {
        Point::new(idx % self.width, idx / self.width)
    }

}

pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    #[allow(dead_code)]
    game: MGame,
    targeter: Targeter,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    GetGame,
    Received(Result<Value, Error>),
    ChangePanel(KeyboardEvent),
    MapAction(KeyboardEvent),
    ItemAction((KeyboardEvent, u64, i32)),
    TargetIndicator((Option<KeyboardEvent>, Option<i32>))
}

#[derive(Serialize)]
struct WsRequest {
    value: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    	Self {
            ws: None,
            link: link,
            game: MGame::new(),
            targeter: Targeter::new(),
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
                let gm: GameMsg = serde_json::from_value(v).unwrap();
                let data: HashMap<String, Value> = serde_json::from_value(gm.data).unwrap();
                //ConsoleService::info(&format!("{:?}", data));
                for (msg, v) in &data {
                    let d = serde_json::from_value(v.clone()).unwrap(); 
                    match msg.trim() {
                        "GAME" => {
                            self.game.map.set_map(d);
                        }
                        "FOV" => {
                            self.game.map.set_fov(d);
                        }
                        "CONTENTS" => {
                            self.game.map.set_contents(d);
                        }
                        "LOG" => {
                            self.game.log.set_logs(d);
                        }
                        "INVENTORY" => {
                            self.game.inventory.set_items(d);
                        }
                        _ => {
                            //ConsoleService::info(&format!("{:?}", gm.d));
                            return false;
                        }
                    }
                }
                true
    		}
            Msg::Received(Err(s)) => {
                let msg = format!("Error when reading data from server: {}\n", &s.to_string());
                ConsoleService::info(&format!("{}", msg));
                false
            }
            Msg::ChangePanel(e) => {
                match e.key_code() {
                    KEY_ESC => { set_focus("map"); false },
                    KEY_I => { set_focus("inventory"); false },
                    _ => false,
                }
            }
            Msg::MapAction(e) => {
                match e.key_code() {
                    KEY_LEFT|KEY_UP|KEY_RIGHT|KEY_DOWN
                    |KEY_Y|KEY_U|KEY_B|KEY_N
                    |KEY_G => { 
                        match self.ws {
                            Some(ref mut task) => {
                                task.send(Ok(e.key()));
                                false
                            }
                            None => false
                        }
                    },
                    _ => false,
                }
            }
            Msg::ItemAction((e, idx, target)) => {
                let event = match e.key_code() {
                    KEY_D => "/drop",
                    KEY_U => "/use",
                    _ => "",
                };
                let action = format!("{} {} {}", event, idx.to_string(), target.to_string());
                match self.ws {
                    Some(ref mut task) => {
                        task.send(Ok(action));
                        false
                    }
                    None => false
                }
            }
            Msg::TargetIndicator((e, n)) => {
                match (e, n) {
                    (None, Some(0)) => {
                        ConsoleService::info("setting up targetter");
                        self.targeter.create(
                            &self.game.map.fov, self.game.map.ppos, self.game.map.width);
                        self.game.map.set_single_target(self.game.map.ppos as usize);
                    }
                    (Some(e), None) => {
                        match self.targeter.try_move(e) {
                            Some(n) => self.game.map.set_single_target(n as usize),
                            None => ()
                        }
                    } 
                    (Some(_e), Some(n)) if n >= 0 => {
                        let idx = self.targeter.idx;
                        let action = format!("/use {} {}", n.to_string(), idx.to_string());
                        match self.ws {
                            Some(ref mut task) => {
                                task.send(Ok(action));
                            }
                            None => ()
                        }
                    }
                    (_, _) => ()
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    //fn rendered(&mut self, first_render: bool) {
        //if first_render {
            //ConsoleService::info("first render");
            //self.link.send_message(Msg::Connect);
            //self.link.send_message(Msg::GetGame);
        //}
    //}

    fn view(&self) -> Html {
    	html! {
            <>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <span style="color: white">{ "Connected: " } { !self.ws.is_none() }</span>
                <button onclick=self.link.callback(|_| Msg::GetGame)>{ "Get Game Dimensions" }</button>
                <Game
                    game=&self.game
                    change_panel_signal=self.link.callback(Msg::ChangePanel)
                    map_action_signal=self.link.callback(Msg::MapAction)
                    item_action_signal=self.link.callback(Msg::ItemAction)
                    target_indicator_signal=self.link.callback(Msg::TargetIndicator)
                />
            </>
    	}
    }
}

fn set_focus(s: &str) {
    document()
        .get_elements_by_class_name(s)
        .get_with_index(0)
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
        .focus()
        .unwrap();
}
