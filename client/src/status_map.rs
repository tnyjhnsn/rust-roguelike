use yew::prelude::*;
use yew::services::ConsoleService;
use super::status::*;

pub struct StatusMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: Vec<String>,
}

impl Component for StatusMap {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        StatusMap { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            ConsoleService::info("STATUSMAP CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("STATUSMAP CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("STATUSMAP UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER STATUSMAP");
        let render_tile = |status: &String| {
            html! {
                <Status status=status />
            }
        };
        html! {
            <div class="status">
                { for self.props.status.iter().map(render_tile) }
            </div>
        }
    }
}

