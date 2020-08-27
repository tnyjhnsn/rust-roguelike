use yew::prelude::*;

use super::model::game_model::*;
use super::map::*;
use super::logs::*;
use super::inventory::*;

pub struct Game {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub game: MGame,
}

impl Component for Game {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Game { props }
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
            <div class="game">
                <div class="holding left-panel">
                    <Inventory inventory=&self.props.game.inventory />
                </div>
                <div class="holding right-panel">
                    <Logs logs=&self.props.game.log dict=&self.props.game.dict />
                </div>
                <div class="holding top-panel">
                    <h1 class="title">{ &self.props.game.title }</h1>
                </div>
                <div class="holding bottom-panel">{ "Bottom Panel" }</div>
                <Map map=&self.props.game.map dict=&self.props.game.dict />
            </div>
        }
    }
}

