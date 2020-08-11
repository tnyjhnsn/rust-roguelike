use yew::prelude::*;
use super::entity::*;

pub struct EntityMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub entities: Vec<String>,
}

impl Component for EntityMap {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        EntityMap { props }
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
        let render_tile = |entity: &String| {
            html! {
                <Entity entity=entity />
            }
        };
        html! {
            <div class="entities">
                { for self.props.entities.iter().map(render_tile) }
            </div>
        }
    }
}

