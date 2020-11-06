use yew::prelude::*;
use super::list_item::*;

pub struct List {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub list: Vec<(i32, i32)>,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
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
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h3>{ "My List" }</h3>
                <ul>
                    {
                        for self.props.list.iter().map(|_| {
                            html! { <ListItem /> }
                        })
                    }
                </ul>
            </div>
        }
    }
}

