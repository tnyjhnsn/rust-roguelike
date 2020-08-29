use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::ConsoleService;
use serde::{Serialize};
use serde_json::Value;
use roguelike_common::*;
use std::collections::HashMap;

use super::model::game_model::*;
use super::game::*;
use super::inventory_dialog::*;

pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    #[allow(dead_code)]
    key_listener: KeyListenerHandle,
    game: MGame,
    show_inv_modal: bool,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    GetGame,
    Received(Result<Value, Error>),
    Pressed(KeyboardEvent),
    Test(KeyboardEvent),
}

#[derive(Serialize)]
struct WsRequest {
    value: String,
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
            Msg::Pressed(e) => {
                if e.key_code() >= 37 && e.key_code() <= 90 {
                    if e.key_code() == 73 {
                        self.show_inv_modal = true;
                        true
                    } else {
                        match self.ws {
                            Some(ref mut task) => {
                                task.send(Ok(e.key()));
                                false
                            }
                            None => false
                        }
                    }
                } else { false }
            }
            Msg::Test(e) => {
                ConsoleService::info(&format!("Key received {}", e.key_code()));
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //let inv_style = if self.show_inv_modal == true { "display: block;" } else { "display: none" };
    	html! {
            <>
                //<div class="modal" style=inv_style>
                    //<div class="modal-content">
                        //<Inventory
                            //inventory=&self.game.inventory
                            //dict=&self.game.dict
                        //>
                    //</div>
                //</div>
                <InventoryDialog show=&self.show_inv_modal onkeydown_signal=self.link.callback(Msg::Test) />
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <span style="color: white">{ "Connected: " } { !self.ws.is_none() }</span>
                <button onclick=self.link.callback(|_| Msg::GetGame)>{ "Get Game Dimensions" }</button>
                <Game game=&self.game show_inv_modal=&self.show_inv_modal />
            </>
    	}
    }
}

