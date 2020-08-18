use yew::prelude::*;
use super::status::*;
use super::viewport::*;

pub struct StatusMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: Vec<i32>,
    pub viewport: Viewport,
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
                { for self.props.viewport.indexes
                    .iter()
                    .map(|i| render_tile(&self.props.status[*i as usize])) }
            </div>
        }
    }
}

