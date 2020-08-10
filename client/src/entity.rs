use yew::prelude::*;
use yew::services::ConsoleService;

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
            //ConsoleService::info("TILE CHANGE True");
            self.props = props;
            true
        } else {
            //ConsoleService::info("TILE CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("ENTITY UPDATE");
        false
    }

    fn view(&self) -> Html {
        //ConsoleService::info("RENDER TILE");
        html! {
            <div class=("tile", &self.props.entity)></div>
        }

    }
}

