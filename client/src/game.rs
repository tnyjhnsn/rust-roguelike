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
    pub change_panel_signal: Callback<KeyboardEvent>,
    pub map_action_signal: Callback<KeyboardEvent>,
    pub item_action_signal: Callback<(KeyboardEvent, u64, i32)>,
    pub target_indicator_signal: Callback<(Option<KeyboardEvent>, Option<i32>)>,
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
        let level_str = format!("Level {}", self.props.game.map.depth);
        html! { 
            <div class="game">
                <div class="holding left-panel">
                    <Armour
                        armour=&self.props.game.armour
                        dict=&self.props.game.dict
                        change_panel_signal=&self.props.change_panel_signal
                    />
                    <Inventory
                        inventory=&self.props.game.inventory
                        fov=&self.props.game.map.fov
                        dict=&self.props.game.dict
                        change_panel_signal=&self.props.change_panel_signal
                        item_action_signal=&self.props.item_action_signal
                        target_indicator_signal=&self.props.target_indicator_signal
                    />
                </div>
                <div class="holding top-panel">
                    <h1 class="title">{ &self.props.game.title }</h1>
                </div>
                <Map
                    map=&self.props.game.map
                    dict=&self.props.game.dict
                    change_panel_signal=&self.props.change_panel_signal
                    map_action_signal=&self.props.map_action_signal
                />
                <div class="holding right-panel">
                    <h3>{ level_str }</h3>
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

