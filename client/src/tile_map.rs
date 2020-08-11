use yew::prelude::*;
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

