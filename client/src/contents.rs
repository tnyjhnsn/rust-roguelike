use yew::prelude::*;
use super::model::dictionary::*;
use yew::services::ConsoleService;

pub struct Contents {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub content: Vec<i32>,
    pub particle: Option<(i32, u64)>,
    pub dict: Dictionary,
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
        let mut tile = String::from("");
        if self.props.content.len() > 0 {
            tile = self.props.dict.get_css(self.props.content[0]);
        }
        if let Some(p) = self.props.particle {
            match p.0 {
                0 => tile = String::from(format!("{} particle-attack", tile)),
                1 => tile = String::from(format!("{} particle-defend", tile)),
                _ => tile = String::from(format!("{} particle-effect", tile)),
            }
            
        }
        html! {
            <div class=("tile", tile)></div>
        }
    }
}

