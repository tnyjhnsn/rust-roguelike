use yew::prelude::*;
use super::tile::*;
use super::model::dictionary::*;
use std::collections::HashMap;

pub struct TileMap {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: Vec<i32>,
    pub contents: HashMap<usize, Vec<i32>>,
    pub particles: HashMap<usize, (i32, u64)>,
    pub viewport: Vec<i32>,
    pub fov: HashMap<usize, f64>,
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
        let render_tile = |status: &i32, content: &Option<Vec<i32>>,
            particle: &Option<(i32, u64)>, opacity: &Option<f64>| {
            html! {
                <Tile
                    status=*status
                    content=content
                    particle=particle
                    opacity=opacity
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
                        &self.check_for_contents(*i as usize),
                        &self.check_for_param(*i as usize, &self.props.particles),
                        &self.check_for_param(*i as usize, &self.props.fov),
                    ))
                }
            </div>
        }
    }
}

impl TileMap {
    fn check_for_param<K, V>(&self, idx: K, h: &HashMap<K, V>) -> Option<V>
    where K: std::cmp::Eq + std::hash::Hash,
          V: std::marker::Copy
    {
        match h.contains_key(&idx) {
            true => Some(*h.get(&idx).unwrap()),
            false => None,
        }
    }
    fn check_for_contents(&self, idx: usize) -> Option<Vec<i32>> {
        match self.props.contents.contains_key(&idx) {
            true => Some(self.props.contents.get(&idx).unwrap().clone()),
            false => None,
        }
    }
}
