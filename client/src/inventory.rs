use yew::prelude::*;
use super::model::inventory_model::*;
use super::model::dictionary::*;
use roguelike_common::*;
use web_sys::{HtmlElement, HtmlCollection};
use yew::utils::document;
use wasm_bindgen::JsCast;

pub struct Inventory {
    link: ComponentLink<Self>,
    selected_item: i32,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
    pub dict: Dictionary,
    pub onkeydown_signal: Callback<KeyboardEvent>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
    GotFocus(FocusEvent),
}

impl Inventory {
    fn cycle_list(&mut self, direction: i32) {
        self.set_selected_item(self.selected_item, "");
        let length = self.get_list_items().length() as i32;
        let selected_idx = ((self.selected_item + direction) % length + length) % length;
        self.set_selected_item(selected_idx, "li-selected");
        self.selected_item = selected_idx;
    }

    fn get_list_items(&self) -> HtmlCollection {
        document()
            .get_elements_by_class_name("inventory-list")
            .get_with_index(0)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
            .children()
    }

    fn set_selected_item(&self, idx: i32, s: &str) {
        match self.get_list_items().get_with_index(idx as u32) {
            Some(item) => item.set_class_name(s),
            None => (),
        }
    }
}

impl Component for Inventory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected_item: 0,
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
        match msg {
            Msg::Pressed(e) => {
                if e.key_code() == KEY_ESC {
                    self.set_selected_item(self.selected_item, "");
                    self.props.onkeydown_signal.emit(e);
                } else if e.key_code() == KEY_DOWN {
                    self.cycle_list(1);
                } else if e.key_code() == KEY_UP {
                    self.cycle_list(-1);
                }
            }
            Msg::GotFocus(_e) => {
                self.selected_item = 0;
                self.set_selected_item(0, "li-selected");
            }
        }
        false
    }

    fn view(&self) -> Html {
        let render_items = |item: &i32| {
            let name = self.props.dict.get_name(*item);
            let css = self.props.dict.get_css(*item);
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
                <ul class="inventory-list">
                { for self.props.inventory.items
                    .iter()
                    .map(render_items) }
                </ul>
            </div>
        }
    }
}

