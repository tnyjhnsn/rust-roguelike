use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;
use super::tile::*;

pub struct TileMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub tiles: Vec<TileType>,
}

impl Component for TileMap {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TileMap { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            ConsoleService::info("TILEMAP CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("TILEMAP CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("TILEMAP UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER TILEMAP");
        let render_tile = |tile: &TileType| {
            html! {
                <Tile tile=*tile />
            }
        };
        html! {
            <div class="tiles">
                { for self.props.tiles.iter().map(render_tile) }
            </div>
        }
    }
}

