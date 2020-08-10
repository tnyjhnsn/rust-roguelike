use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;

pub struct Tile {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub tile: TileType,
}

impl Component for Tile {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tile { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            //ConsoleService::info("TILE CHANGE True");
            self.props = props;
            true
        } else {
            //ConsoleService::info("TILE CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("TILE UPDATE");
        false
    }

    fn view(&self) -> Html {
        //ConsoleService::info("RENDER TILE");
        let tile = if self.props.tile == TileType::Wall { "wall" } else { "floor" };
        html! {
            <div class=("tile", tile)></div>
        }

    }
}

