use yew::prelude::*;

use super::app::*;
use super::tile_map::*;
use super::entity_map::*;
use super::status_map::*;

pub struct Dungeon {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub map: Map,
}

impl Component for Dungeon {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Dungeon { props }
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
            <div class="dungeon">
                <TileMap tiles=&self.props.map.tiles viewport=&self.props.map.viewport />
                <EntityMap entities=&self.props.map.entities viewport=&self.props.map.viewport />
                <StatusMap status=&self.props.map.status viewport=&self.props.map.viewport />
            </div>
        }
    }
}

