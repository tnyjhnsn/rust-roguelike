use yew::prelude::*;

use super::model::map_model::*;
use super::model::dictionary::*;

use super::tile_map::*;
use super::contents_map::*;
use super::status_map::*;

pub struct Map {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub map: MMap,
    pub dict: Dictionary,
    pub onkeydown_signal: Callback<KeyboardEvent>,
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
                //ConsoleService::info("Pressed in dialog");
                self.props.onkeydown_signal.emit(e);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! { 
            <div
                class="map"
                tabindex="0"
                onkeydown=self.link.callback(Msg::Pressed)
            >
                <TileMap tiles=&self.props.map.tiles viewport=&self.props.map.viewport />
                <ContentsMap
                    contents=&self.props.map.contents
                    dict=&self.props.dict
                    viewport=&self.props.map.viewport />
                <StatusMap status=&self.props.map.status viewport=&self.props.map.viewport />
            </div>
        }
    }
}

