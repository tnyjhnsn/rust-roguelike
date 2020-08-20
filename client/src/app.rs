use anyhow::Error;
use yew::prelude::*;
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::services::keyboard::{KeyboardService, KeyListenerHandle};
use yew::services::ConsoleService;
use serde::{Serialize};
use serde_json::Value;
use roguelike_common::*;

use super::model::game_model::*;
use super::game::*;

pub struct Model {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Model>,
    #[allow(dead_code)]
    key_listener: KeyListenerHandle,
    game: MGame,
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
                match gm.msg.trim() {
                    "GAME" => {
                        self.game.map.set_map(gm.data);
                        true
                    }
                    "FOV" => {
                        self.game.map.set_fov(gm.data);
                        true
                    }
                    "LOG" => {
                        self.game.log.set_logs(gm.data);
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
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <span style="color: white">{ "Connected: " } { !self.ws.is_none() }</span>
                <button onclick=self.link.callback(|_| Msg::GetGame)>{ "Get Game Dimensions" }</button>
                <Game game=&self.game />
            </>
    	}
    }
}

