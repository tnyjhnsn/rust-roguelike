use yew::prelude::*;
use super::model::inventory_model::*;
use super::model::dictionary::*;
use roguelike_common::*;
use web_sys::{HtmlElement, HtmlCollection};
use yew::utils::document;
use wasm_bindgen::JsCast;

pub struct Inventory {
    link: ComponentLink<Self>,
    list_items: Option<HtmlCollection>,
    selected_item: i32,
    targeting: bool,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
    pub fov: Vec<usize>,
    pub dict: Dictionary,
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub item_action_signal: Callback<(KeyboardEvent, u64, i32)>,
    pub target_indicator_signal: Callback<(Option<KeyboardEvent>, Option<i32>)>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
    GotFocus(FocusEvent),
}

impl Inventory {
    fn cycle_list(&mut self, direction: i32) {
        match &self.list_items {
            Some(items) => {
                let length = items.length() as i32;
                if length == 0 { return; }
                self.set_selected_item(self.selected_item, "");
                let selected_idx = ((self.selected_item + direction) % length + length) % length;
                self.set_selected_item(selected_idx, "li-selected");
                self.selected_item = selected_idx;
            }
            None => (),
        }
    }

    fn get_list_items(&self) -> HtmlCollection {
        document()
            .get_elements_by_class_name("selectable-list")
            .get_with_index(0)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .children()
    }

    fn set_selected_item(&self, idx: i32, s: &str) {
        match &self.list_items {
            Some(items) => {
                items.get_with_index(idx as u32).unwrap().set_class_name(s);
            }
            None => ()
        }
    }
}

impl Component for Inventory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            list_items: None,
            selected_item: 0,
            targeting: false,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
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
                        }
                        KEY_ENTER => {
                            let idx = self.props.inventory.items[self.selected_item as usize].1;
                            self.props.target_indicator_signal.emit((Some(e), Some(idx as i32)));
                            self.targeting = false;
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
            match msg {
                Msg::Pressed(e) => {
                    match e.key_code() {
                        KEY_ESC => {
                            //self.set_selected_item(self.selected_item, "");
                            self.props.change_panel_signal.emit(e);
                        },
                        KEY_DOWN =>  self.cycle_list(1),
                        KEY_UP => self.cycle_list(-1),
                        KEY_D => {
                            match self.list_items {
                                Some(_) => {
                                    let idx = self.props.inventory.items[self.selected_item as usize].1;
                                    self.props.item_action_signal.emit((e, idx, -1));
                                }
                                None => (),
                            }
                        }
                        KEY_U => {
                            match self.list_items {
                                Some(_) => {
                                    let (item, idx) = self.props.inventory.items[self.selected_item as usize];
                                    if item < 2100 || item >= 3000 {
                                        self.props.item_action_signal.emit((e, idx, -1));
                                    } else {
                                        self.targeting = true;
                                        self.props.target_indicator_signal.emit((None, Some(0)));
                                    }
                                }
                                None => (),
                            }
                        }
                        _ => (),
                    }
                }
                Msg::GotFocus(_e) => {
                    match self.props.inventory.items.len() {
                        0 => (),
                        _ => {
                            self.list_items = Some(self.get_list_items());
                            self.selected_item = 0;
                            self.set_selected_item(0, "li-selected");
                        }
                    }
                }
            }
            false
        }
    }

    fn view(&self) -> Html {
        let render_items = |item: &(i32, u64)| {
            let name = self.props.dict.get_name(item.0);
            let css = self.props.dict.get_css(item.0);
            html! {
                <li>
                    <div class="flex-wrap">
                        <div class=("tile", css)></div>
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
                <ul class="selectable-list">
                { for self.props.inventory.items
                    .iter()
                    .map(render_items) }
                </ul>
            </div>
        }
    }
}

