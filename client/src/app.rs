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

pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    #[allow(dead_code)]
    game: MGame,
    show_inv_modal: bool,
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
    TargetIndicator(usize)
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
            show_inv_modal: false,
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
            Msg::TargetIndicator(i) => {
                self.game.map.set_single_target(self.game.map.fov[i]); 
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
                    show_inv_modal=&self.show_inv_modal
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
