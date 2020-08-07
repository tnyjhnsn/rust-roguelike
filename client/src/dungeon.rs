use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;

use super::level::*;

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
            ConsoleService::info("DUNGEON CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("DUNGEON CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("DUNGEON UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER DUNGEON");
        html! { 
            <div class="dungeon">
                <Level tiles=&self.props.map.tiles />
            </div>
        }
    }
}

