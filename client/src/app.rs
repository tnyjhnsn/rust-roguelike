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
use super::armour::*;
use super::inventory::*;
use super::map::*;
use super::logs::*;
use super::stats::*;
use super::dialog::*;
use super::list::*;
use super::list_item::*;

pub struct Game {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    game: MGame,
    show_dialog: bool,
}

pub enum Msg {
    Connect,
    Disconnected,
    Ignore,
    GetCampaign,
    Received(Result<Value, Error>),
    ChangePanel(KeyboardEvent),
    MapAction(KeyboardEvent),
    ItemAction((KeyboardEvent, i32, i32)),
    TargetIndicator((Option<KeyboardEvent>, Option<i32>)),
    ShowDialog,
}

#[derive(Serialize)]
struct WsRequest {
    value: String,
}

impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    	Self {
            ws: None,
            link: link,
            game: MGame::new(),
            show_dialog: false,
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
    			false
    		}
    		Msg::Ignore => {
    			false
    		}
    		Msg::GetCampaign => {
    			match self.ws {
    				Some(ref mut task) => {
    					task.send(Ok(String::from("/campaign")));
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
                        "MAP" => {
                            self.game.map.set_map(d);
                            set_focus("map");
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
                        "ARMOUR" => {
                            self.game.armour.set_items(d);
                        }
                        "PARTICLES" => {
                            self.game.map.set_particles(d);
                        }
                        "COMBAT_STATS" => {
                            self.game.stats.set_combat(d);
                        }
                        "ATTR_STATS" => {
                            self.game.stats.set_attributes(d);
                        }
                        "LEVEL_STATS" => {
                            self.game.stats.set_level_xp(d);
                        }
                        "ENCUMBRANCE" => {
                            self.game.stats.set_encumbrance(d);
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
                    KEY_A => { set_focus("armour"); false },
                    KEY_X => { self.show_dialog = false; true },
                    _ => false,
                }
            }
            Msg::MapAction(e) => {
                match e.key_code() {
                    KEY_LEFT|KEY_UP|KEY_RIGHT|KEY_DOWN
                    |KEY_Y|KEY_U|KEY_B|KEY_N
                    |KEY_G|KEY_GT|KEY_LT => { 
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
                    KEY_R => "/remove",
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
                        self.game.map.reset_targeter();
                        self.game.map.set_single_target(self.game.map.ppos as usize);
                    }
                    (Some(e), None) => {
                        match self.game.map.try_move(e.key_code()) {
                            Some(n) => self.game.map.set_single_target(n as usize),
                            None => ()
                        }
                    } 
                    (Some(_e), Some(n)) if n >= 0 => {
                        let idx = self.game.map.target;
                        let action = format!("/use {} {}", n.to_string(), idx.to_string());
                        match self.ws {
                            Some(ref mut task) => {
                                task.send(Ok(action));
                            }
                            None => ()
                        }
                        self.game.map.clear_targeter();
                    }
                    (_, _) => {}
                }
                true
            }
            Msg::ShowDialog => {
                self.show_dialog = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { 
            <div class="game">
                <Dialog
                    show=self.show_dialog
                    change_panel_signal=self.link.callback(Msg::ChangePanel)
                />
                <div class="holding left-panel">
                    <Stats
                        stats=&self.game.stats
                    />
                    <Armour
                        armour=&self.game.armour
                        dict=&self.game.dict
                        change_panel_signal=self.link.callback(Msg::ChangePanel)
                        item_action_signal=self.link.callback(Msg::ItemAction)
                    />
                    <Inventory
                        inventory=&self.game.inventory
                        dict=&self.game.dict
                        change_panel_signal=self.link.callback(Msg::ChangePanel)
                        item_action_signal=self.link.callback(Msg::ItemAction)
                        target_indicator_signal=self.link.callback(Msg::TargetIndicator)
                    />
                    <List list=&self.game.inventory.items />
                </div>
                <div class="holding top-panel">
                    <h1 class="title">{ &self.game.title }</h1>
                </div>
                <Map
                    map=&self.game.map
                    dict=&self.game.dict
                    change_panel_signal=self.link.callback(Msg::ChangePanel)
                    map_action_signal=self.link.callback(Msg::MapAction)
                />
                <div class="holding right-panel">
                    <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                    <span style="color: white">{ " Connected: " } { !self.ws.is_none() }</span>
                    <button onclick=self.link.callback(|_| Msg::GetCampaign)>{ "Get Campaign" }</button>
                    <button onclick=self.link.callback(|_| Msg::ShowDialog)>{ "Show Dialog" }</button>
                    <Logs
                        logs=&self.game.log
                        dict=&self.game.dict
                    />
                </div>
                <div class="holding bottom-panel">{ "Bottom Panel" }</div>
            </div>
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
