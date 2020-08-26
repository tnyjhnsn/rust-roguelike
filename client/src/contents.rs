use yew::prelude::*;

pub struct Contents {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub content: Vec<i32>
}

impl Component for Contents {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Contents { props }
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
        let tile = if self.props.content.len() > 0 { "player-m" } else { "" };
        html! {
            <div class=("tile", tile)></div>
        }

    }
}

