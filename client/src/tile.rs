use yew::prelude::*;
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
        let tile = match self.props.tile {
            TileType::Wall => "",
            TileType::Floor => "",
            TileType::DownStairs => "stairs-down",
        };
        html! {
            <div class=("tile", tile)></div>
        }

    }
}

