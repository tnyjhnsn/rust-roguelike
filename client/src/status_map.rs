use yew::prelude::*;
use super::status::*;

pub struct StatusMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: Vec<i32>,
}

impl Component for StatusMap {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        StatusMap { props }
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
        let render_tile = |status: &i32| {
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

