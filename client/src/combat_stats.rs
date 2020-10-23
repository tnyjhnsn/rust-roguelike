use yew::prelude::*;

pub struct CombatStats {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: &'static str,
    pub colour: &'static str,
    pub stats: (i32, i32),
}

impl Component for CombatStats {
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
        let (current, max) = self.props.stats;
        let value = match max {
            0 => 0.0,
            _ => (current as f64 / max as f64) * 100.0,
        };
        let style = format!("background-color:{};width:{}%", self.props.colour, value);
        html! {
            <>
                <div>{ format!("{} {}/{}", self.props.title, current, max) }</div>
                <div class="combat-stats-wrapper">
                    <div class="combat-stats" style=style></div>
                </div>
            </>
        }
    }
}

