use yew::prelude::*;
use super::model::armour_model::*;
use super::model::dictionary::*;
use roguelike_common::*;
use std::cmp::{max};

pub struct Armour {
    link: ComponentLink<Self>,
    selected_idx: i32,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub armour: MArmour,
    pub dict: Dictionary,
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub item_action_signal: Callback<(KeyboardEvent, i32, i32)>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
    GotFocus(FocusEvent),
}

impl Armour {
    fn cycle_list(&mut self, direction: i32) {
        let len = self.props.armour.items.len() as i32;
        match len {
            0 => (),
            _ => self.selected_idx = ((self.selected_idx + direction) % len + len) % len,
        }
    }
}

impl Component for Armour {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            selected_idx: -1,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.armour != props.armour {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let len = self.props.armour.items.len();
        match msg {
            Msg::Pressed(e) => {
                match e.key_code() {
                    KEY_ESC|KEY_I => {
                        self.props.change_panel_signal.emit(e);
                        self.selected_idx = -1;
                    },
                    KEY_DOWN =>  self.cycle_list(1),
                    KEY_UP => self.cycle_list(-1),
                    KEY_R => {
                        match len {
                            0 => (),
                            _ => {
                                let idx = self.props.armour.items[self.selected_idx as usize].1;
                                self.props.item_action_signal.emit((e, idx, -1));
                                self.selected_idx = max(0, self.selected_idx - 1);
                            }
                        }
                    }
                    _ => (),
                }
            }
            Msg::GotFocus(_e) => {
                match len {
                    0 => (),
                    _ => self.selected_idx = 0,
                }
            }
        }
        true
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
                class="armour"
                tabindex="0"
                onkeydown=self.link.callback(Msg::Pressed)
                onfocus= self.link.callback(Msg::GotFocus)
            >
                <h3>{ "Armour" }</h3>
                <ul class="armour-list">
                { for self.props.armour.items
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| render_items(idx, item)) }
                </ul>
            </div>
        }
    }
}

