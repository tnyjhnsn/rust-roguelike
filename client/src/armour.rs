use yew::prelude::*;
use super::model::dictionary::*;
use roguelike_common::*;

pub struct Armour {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub armour: String,
    pub dict: Dictionary,
    pub change_panel_signal: Callback<KeyboardEvent>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
}

impl Component for Armour {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Pressed(e) => {
                match e.key_code() {
                    KEY_ESC|KEY_I => {
                        self.props.change_panel_signal.emit(e);
                    }
                    _ => ()
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                class="armour"
                tabindex="0"
                onkeydown=self.link.callback(Msg::Pressed)
            >
                <h3>{ &self.props.armour }</h3>
            </div>
        }
    }
}

