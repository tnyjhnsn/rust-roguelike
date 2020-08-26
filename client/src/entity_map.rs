use yew::prelude::*;
use super::entity::*;

pub struct EntityMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub contents: Vec<Vec<i32>>,
    pub viewport: Vec<i32>,
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
        let render_tile = |content: &Vec<i32>| {
            html! {
                <Entity content=content />
            }
        };
        html! {
            <div class="entities">
                { for self.props.viewport
                    .iter()
                    .map(|i| render_tile(&self.props.contents[*i as usize])) }
            </div>
        }
    }
}

