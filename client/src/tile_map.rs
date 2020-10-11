use yew::prelude::*;
use super::tile::*;
use super::model::dictionary::*;

pub struct TileMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: Vec<i32>,
    pub contents: Vec<Vec<i32>>,
    pub particles: Vec<Option<(i32, u64)>>,
    pub viewport: Vec<i32>,
    pub background: String,
    pub dict: Dictionary,
}

impl Component for TileMap {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TileMap { props }
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
        let render_tile = |status: &i32, content: &Vec<i32>, particle: &Option<(i32, u64)>| {
            html! {
                <Tile
                    status=*status
                    content=content
                    particle=particle
                    dict=&self.props.dict
                />
            }
        };
        html! {
            <div class="tiles" style=&self.props.background>
                { for self.props.viewport
                    .iter()
                    .map(|i| render_tile(
                            &self.props.status[*i as usize],
                            &self.props.contents[*i as usize],
                            &self.props.particles[*i as usize],
                            )) }
            </div>
        }
    }
}

