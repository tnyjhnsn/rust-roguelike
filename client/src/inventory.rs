use yew::prelude::*;
use super::model::inventory_model::*;
use super::model::dictionary::*;

pub struct Inventory {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
    pub dict: Dictionary,
    pub onkeydown_signal: Callback<KeyboardEvent>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
}

impl Component for Inventory {
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
                if e.key_code() == 27 {
                    self.props.onkeydown_signal.emit(e);
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        let render_items = |item: &i32| {
            let name = self.props.dict.get_name(*item);
            let css = self.props.dict.get_css(*item);
            html! {
                <li>
                    <div class="flex-wrap">
                        <div class=("tile", css)></div>
                        <div class="content">{ name }</div>
                    </div>
                </li>
            }
        };
        html! {
            <div
                class="inventory"
                tabindex="0"
                onkeydown=self.link.callback(Msg::Pressed)
            >
                <h3>{ "Inventory" }</h3>
                <ul>
                { for self.props.inventory.items
                    .iter()
                    .map(render_items) }
                </ul>
            </div>
        }
    }
}

