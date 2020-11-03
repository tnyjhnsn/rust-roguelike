use yew::prelude::*;
use super::model::inventory_model::*;
use super::model::dictionary::*;
use roguelike_common::*;
use std::cmp::{max};

pub struct Inventory {
    link: ComponentLink<Self>,
    selected_idx: i32,
    targeting: bool,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
    pub dict: Dictionary,
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub item_action_signal: Callback<(KeyboardEvent, i32, i32)>,
    pub target_indicator_signal: Callback<(Option<KeyboardEvent>, Option<i32>)>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
    GotFocus(FocusEvent),
}

impl Inventory {
    fn cycle_list(&mut self, direction: i32) {
        let len = self.props.inventory.items.len() as i32;
        match len {
            0 => (),
            _ => {
                self.selected_idx = ((self.selected_idx + direction) % len + len) % len;
            }
        }
    }
}

impl Component for Inventory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected_idx: -1,
            targeting: false,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.inventory != props.inventory {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.targeting {
            match msg {
                Msg::Pressed(e) => {
                    match e.key_code() {
                        KEY_ESC => {
                            //TODO targeter cleanup
                            self.targeting = false;
                            self.selected_idx = -1;
                        }
                        KEY_ENTER => {
                            let idx = self.props.inventory.items[self.selected_idx as usize].1;
                            self.props.target_indicator_signal.emit((Some(e), Some(idx as i32)));
                            self.targeting = false;
                            self.selected_idx = max(0, self.selected_idx - 1);
                        }
                        KEY_LEFT|KEY_RIGHT|KEY_UP|KEY_DOWN
                        |KEY_Y|KEY_U|KEY_B|KEY_N => {
                            self.props.target_indicator_signal.emit((Some(e), None));
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
            false
        } else {
            let len = self.props.inventory.items.len();
            match msg {
                Msg::Pressed(e) => {
                    match e.key_code() {
                        KEY_ESC|KEY_A => {
                            self.props.change_panel_signal.emit(e);
                            self.selected_idx = -1;
                        }
                        KEY_DOWN =>  self.cycle_list(1),
                        KEY_UP => self.cycle_list(-1),
                        KEY_D => {
                            match len {
                                0 => (),
                                _ => {
                                    let idx = self.props.inventory.items[self.selected_idx as usize].1;
                                    self.props.item_action_signal.emit((e, idx, -1));
                                    self.selected_idx = max(0, self.selected_idx - 1);
                                }
                            }
                        }
                        KEY_U => {
                            match len {
                                0 => (),
                                _ => {
                                    let (item, idx) = self.props.inventory.items[self.selected_idx as usize];
                                    if item < 2100 || item >= 3000 {
                                        self.props.item_action_signal.emit((e, idx, -1));
                                    } else {
                                        self.targeting = true;
                                        self.props.target_indicator_signal.emit((None, Some(0)));
                                    }
                                }
                            }
                        }
                        _ => ()
                    }
                }
                Msg::GotFocus(_e) => {
                    match len {
                        0 => (),
                        _ => {
                            self.selected_idx = 0;
                        }
                    }
                }
            }
            true
        }
    }

    fn view(&self) -> Html {
        let render_items = |idx: usize, item: &(i32, i32)| {
            let name = self.props.dict.get_name(item.0);
            let css = self.props.dict.get_css(item.0);
            let selected = if idx == self.selected_idx as usize { "li-selected" } else { "" };
            html! {
                <li>
                    <div class=("flex-wrap", selected)>
                        <div class="tile-box">
                            <div class=("tile", css)></div>
                        </div>
                        <div class="content">{ name }</div>
                    </div>
                </li>
            }
        };
        html! {
            <div
                class="inventory"
                tabindex="0"
                onkeydown=self.link.callback(Msg::Pressed)
                onfocus= self.link.callback(Msg::GotFocus)
            >
                <h3>{ "Inventory" }</h3>
                <ul class="inventory-list">
                { for self.props.inventory.items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| render_items(idx, item)) }
                </ul>
            </div>
        }
    }
}

