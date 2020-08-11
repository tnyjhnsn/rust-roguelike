use yew::prelude::*;

pub struct Entity {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub entity: String
}

impl Component for Entity {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Entity { props }
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
            <div class=("tile", &self.props.entity)></div>
        }

    }
}

