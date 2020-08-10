use yew::prelude::*;
use yew::services::ConsoleService;
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
            ConsoleService::info("ENTITYMAP CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("ENTITYMAP CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("ENTITYMAP UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER ENTITYMAP");
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

