use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;

pub struct Level {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub tiles: Vec<TileType>,
}

impl Component for Level {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Level { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            ConsoleService::info("LEVEL CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("LEVEL CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("LEVEL UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER LEVEL");
        let render_tile = |tile: &TileType| {
            let tile_status = if *tile == TileType::Wall { "wall" } else { "floor" };
            html! {
                <div class=("tile", tile_status)></div>
            }

        };
        html! {
            <div class="level">
                { for self.props.tiles.iter().map(render_tile) }
            </div>
        }
    }
}

