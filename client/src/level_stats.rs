use yew::prelude::*;

pub struct LevelStats {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: &'static str,
    pub colour: &'static str,
    pub stats: (i32, i32),
}

impl Component for LevelStats {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
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
        let (level, xp) = self.props.stats;
        let xp_level_start = (level - 1) * 1000;
        let xp_level_end = level * 1000;
        let value = ((xp - xp_level_start) as f64 / xp_level_end as f64) * 100.0;
        let style = format!("background-color:{};width:{}%", self.props.colour, value);
        html! {
            <>
                <div>{ format!("{} {}", self.props.title, level) }</div>
                <div class="combat-stats-wrapper">
                    <div class="combat-stats" style=style></div>
                </div>
            </>
        }
    }
}

