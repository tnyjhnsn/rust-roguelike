use yew::prelude::*;

use super::model::map_model::*;
use super::model::dictionary::*;

use super::tile_map::*;

use roguelike_common::*;

pub struct Map {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub map: MMap,
    pub dict: Dictionary,
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub map_action_signal: Callback<KeyboardEvent>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
}

impl Component for Map {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.map != props.map {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Pressed(e) => {
                match e.key_code() {
                    KEY_I|KEY_A => self.props.change_panel_signal.emit(e),
                    _ => self.props.map_action_signal.emit(e),
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let mut background = String::new();
        if self.props.map.viewport.len() > 0 {
            let background_img = format!("background-image: url('{}.jpg')", self.props.map.key);
            let p = self.props.map.idx_xy(self.props.map.viewport[0]);
            let background_pos = format!("background-position: -{}px -{}px;", p.x * 32, p.y * 32);
            background = format!("{}; {}", background_img, background_pos);
        }
        html! { 
            <div
                class="map"
                tabindex="0"
                onkeydown=self.link.callback(
                    |e: KeyboardEvent| { e.prevent_default(); Msg::Pressed(e) })
            >
                <TileMap
                    status=&self.props.map.status
                    contents=&self.props.map.contents
                    particles=&self.props.map.particles
                    viewport=&self.props.map.viewport
                    fov=&self.props.map.fov
                    background=&background
                    dict=&self.props.dict
                />
            </div>
        }
    }
}

