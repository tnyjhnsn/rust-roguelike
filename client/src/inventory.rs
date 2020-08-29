use yew::prelude::*;
use super::model::inventory_model::*;
use super::model::dictionary::*;

pub struct Inventory {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
    pub dict: Dictionary,
}

impl Component for Inventory {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Inventory { props }
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
            <div class="inventory">
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

