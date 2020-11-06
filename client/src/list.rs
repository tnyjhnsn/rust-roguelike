use yew::prelude::*;
use yew::services::ConsoleService;

pub struct List {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        ConsoleService::info(&format!("children {:?}", self.props.children));
        html! {
            <div>
                <h3>{ "My List" }</h3>
                <ul>
                { self.props.children.clone() }
                </ul>
                <div>{ self.props.children.len() }</div>
            </div>
        }
    }
}

