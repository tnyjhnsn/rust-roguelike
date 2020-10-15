use yew::prelude::*;
use roguelike_common::*;
use super::model::dictionary::*;

pub struct Tile {
    link: ComponentLink<Self>,
    animation_ended: bool,
    props: Props,
}

pub enum Msg {
    AnimationEnd(AnimationEvent),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: i32,
    pub content: Option<Vec<i32>>,
    pub particle: Option<(i32, u64)>,
    pub dict: Dictionary,
}

impl Component for Tile {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Tile {
            link,
            animation_ended: false,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            self.animation_ended = false;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AnimationEnd(_e) => {
                self.animation_ended = true;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=("tile",
                    self.get_status_css(), 
                    self.get_contents_css(), 
                    self.get_particle_css())
                onanimationend=self.link.callback(Msg::AnimationEnd)
            ></div>
        }
    }
}

impl Tile {
    fn get_status_css(&self) -> String {
        let mut status = String::from("not-seen");
        if self.props.status & SEEN != 0 { status = String::from("seen") };
        if self.props.status & VISIBLE != 0 { status = String::from("visible") };
        if self.props.status & TARGETED != 0 { status = String::from("targeted") };
        status
    }
    fn get_contents_css(&self) -> String {
        let mut contents = String::from("");
        if let Some(c) = &self.props.content {
            contents = self.props.dict.get_css(c[0]);
        }
        contents
    }
    fn get_particle_css(&self) -> String {
        let mut effects = String::from("");
        if !self.animation_ended {
            if let Some(p) = self.props.particle {
                match p.0 {
                    0 => effects = String::from("particle-attack"),
                    1 => effects = String::from("particle-defend"),
                    ITEM_ACID_RAIN => effects = String::from("particle-acid-rain"),
                    ITEM_DRAGON_BREATH => effects = String::from("particle-dragon-breath"),
                    _ => effects = String::from("particle-effect"),
                }
            }
        }
        effects
    }
}
