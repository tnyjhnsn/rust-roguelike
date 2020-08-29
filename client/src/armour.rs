use yew::prelude::*;
use super::model::dictionary::*;

pub struct Armour {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub armour: String,
    pub dict: Dictionary,
}

impl Component for Armour {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
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
            <div class="armour">
                <h3>{ &self.props.armour }</h3>
            </div>
        }
    }
}

