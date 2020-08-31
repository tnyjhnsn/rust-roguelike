use yew::prelude::*;

use super::model::game_model::*;
use super::armour::*;
use super::inventory::*;
use super::map::*;
use super::logs::*;

pub struct Game {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub game: MGame,
    pub show_inv_modal: bool,
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub player_action_signal: Callback<KeyboardEvent>,
    pub drop_item_signal: Callback<u64>,
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
                    <Armour
                        armour=&self.props.game.armour
                        dict=&self.props.game.dict
                    />
                    <Inventory
                        inventory=&self.props.game.inventory
                        dict=&self.props.game.dict
                        change_panel_signal=&self.props.change_panel_signal
                        drop_item_signal=&self.props.drop_item_signal
                    />
                </div>
                <div class="holding top-panel">
                    <h1 class="title">{ &self.props.game.title }</h1>
                </div>
                <Map
                    map=&self.props.game.map
                    dict=&self.props.game.dict
                    change_panel_signal=&self.props.change_panel_signal
                    player_action_signal=&self.props.player_action_signal
                />
                <div class="holding right-panel">
                    <Logs
                        logs=&self.props.game.log
                        dict=&self.props.game.dict
                    />
                </div>
                <div class="holding bottom-panel">{ "Bottom Panel" }</div>
            </div>
        }
    }
}

