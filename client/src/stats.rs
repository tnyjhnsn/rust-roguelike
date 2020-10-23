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
                { self.get_combat_stats("Health", "red",
                    calc_combat_stats(self.props.stats.health)) }
                { self.get_combat_stats("Mana", "blue",
                    calc_combat_stats(self.props.stats.mana)) }
                <div class="attr-stats-wrapper">
                    { self.get_attr_stats("Might", self.props.stats.might) }
                    { self.get_attr_stats("Fitness", self.props.stats.fitness) }
                    { self.get_attr_stats("Quickness", self.props.stats.quickness) }
                    { self.get_attr_stats("Intelligence", self.props.stats.intelligence) }
                </div>
            </div>
        }
    }
}

impl Stats {
    fn get_combat_stats(&self, title: &str, colour: &str, value: i32) -> Html {
        let style = format!("background-color:{};width:{}%", colour, value);
        html! {
            <>
                <h4>{ title }</h4>
                <div class="combat-stats-wrapper">
                    <div class="combat-stats" style=style></div>
                </div>
            </>
        }
    }
    fn get_attr_stats(&self, title: &str, attr: (i32, i32, i32)) -> Html {
        let a = calc_attributes(attr);
        let style = if a.1 < 0 { "color:red" } else { "" };
        html! {
            <div class="attr-stats">
                <div class="a-title">{title}</div>
                <div class="value">{a.0}</div>
                <div class="bonus" style=style>{a.1}</div>
            </div>
        }
    }
}

pub fn calc_attributes(attr: (i32, i32, i32)) -> (i32, i32) {
    (attr.0 + attr.1, attr.2)
}

fn calc_combat_stats(stats: (i32, i32)) -> i32 {
    match stats.1 {
        0 => 0,
        _ => (stats.0 / stats.1) * 100,
    }
}

