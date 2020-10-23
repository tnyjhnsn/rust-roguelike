use yew::prelude::*;
use super::model::stats_model::*;

pub struct Stats {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub stats: MStats,
}

impl Component for Stats {
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
        html! {
            <div class="stats">
                <h3>{ "Stats" }</h3>
                { get_combat_stats("Health", "red",
                    calc_combat_stats(self.props.stats.health)) }
                { get_combat_stats("Mana", "blue",
                    calc_combat_stats(self.props.stats.mana)) }
                <div class="attr-stats-wrapper">
                    { get_attr_stats("Might", self.props.stats.might) }
                    { get_attr_stats("Fitness", self.props.stats.fitness) }
                    { get_attr_stats("Quickness", self.props.stats.quickness) }
                    { get_attr_stats("Intelligence", self.props.stats.intelligence) }
                </div>
            </div>
        }
    }
}

fn get_combat_stats(title: &str, colour: &str, value: i32) -> Html {
    let style = format!("background-color:{};width:{}%", colour, value);
    html! {
        <>
            <div>{ title }</div>
            <div class="combat-stats-wrapper">
                <div class="combat-stats" style=style></div>
            </div>
        </>
    }
}

fn get_attr_stats(title: &str, attr: (i32, i32, i32)) -> Html {
    let (modified, bonus) = (attr.0 + attr.1, attr.2);
    let style = if bonus < 0 { "color:red" } else { "" };
    html! {
        <div class="attr-stats">
            <div class="a-title">{ title }</div>
            <div class="value">{ modified }</div>
            <div class="bonus" style=style>{ bonus }</div>
        </div>
    }
}

fn calc_combat_stats((current, max): (i32, i32)) -> i32 {
    match max {
        0 => 0,
        _ => (current / max) * 100,
    }
}

