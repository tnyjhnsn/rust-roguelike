use yew::prelude::*;
use super::model::inventory_model::*;

pub struct Inventory {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub inventory: MInventory,
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
            html! {
                //<div>{inventory.get_item(item)}</div>
                <div>{ format!("Item {}", item) }</div>
            }
        };
        html! {
            <div class="log">
                { for self.props.inventory.items
                    .iter()
                    .map(render_items) }
            </div>
        }
    }
}

