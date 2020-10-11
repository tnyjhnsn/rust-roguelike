use yew::prelude::*;
use roguelike_common::*;
use super::model::dictionary::*;

pub struct Tile {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub tile: TileType,
    pub status: i32,
    pub content: Vec<i32>,
    pub particle: Option<(i32, u64)>,
    pub dict: Dictionary,
}

impl Component for Tile {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tile { props }
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
        html! {
            <div class=("tile",
                        self.get_tile_css(),
                        self.get_status_css(), 
                        self.get_contents_css(), 
                        self.get_particle_css())>
            </div>
        }
    }
}

impl Tile {
    fn get_tile_css(&self) -> String {
        match self.props.tile {
            TileType::DownStairs => String::from("stairs-down"),
            _ => String::from("")
        }
    }
    fn get_status_css(&self) -> String {
        let mut status = String::from("not-seen");
        if self.props.status & SEEN != 0 { status = String::from("seen") };
        if self.props.status & VISIBLE != 0 { status = String::from("visible") };
        if self.props.status & TARGETED != 0 { status = String::from("targeted") };
        status
    }
    fn get_contents_css(&self) -> String {
        let mut contents = String::from("");
        if self.props.content.len() > 0 {
            contents = self.props.dict.get_css(self.props.content[0]);
        }
        contents
    }
    fn get_particle_css(&self) -> String {
        let mut effects = String::from("");
        if let Some(p) = self.props.particle {
            match p.0 {
                0 => effects = String::from("particle-attack"),
                1 => effects = String::from("particle-defend"),
                _ => effects = String::from("particle-effect"),
            }
        }
        effects
    }
}
