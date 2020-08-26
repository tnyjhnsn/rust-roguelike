use yew::prelude::*;

use super::model::map_model::*;

use super::tile_map::*;
use super::contents_map::*;
use super::status_map::*;

pub struct Map {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub map: MMap,
}

impl Component for Map {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Map { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { 
            <div>
                <TileMap tiles=&self.props.map.tiles viewport=&self.props.map.viewport />
                <ContentsMap contents=&self.props.map.contents viewport=&self.props.map.viewport />
                <StatusMap status=&self.props.map.status viewport=&self.props.map.viewport />
            </div>
        }
    }
}

