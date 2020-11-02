use yew::prelude::*;
use super::combat_stats::*;
use super::attr_stats::*;
use super::level_stats::*;
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
        let level_xp = (self.props.stats.level, self.props.stats.xp);
        let (weight, capacity, penalty) = &self.props.stats.encumbrance;
        html! {
            <div class="stats">
                <h3>{ "Stats" }</h3>
                <CombatStats title="Health" colour="red" stats=&self.props.stats.health />
                <CombatStats title="Mana" colour="blue" stats=&self.props.stats.mana />
                <LevelStats title="Level" colour="gold" stats=level_xp />
                <div class="attr-stats-wrapper">
                    <AttrStats title="Might" stats=&self.props.stats.might />
                    <AttrStats title="Fitness" stats=&self.props.stats.fitness />
                    <AttrStats title="Quickness" stats=&self.props.stats.quickness />
                    <AttrStats title="Intelligence" stats=&self.props.stats.intelligence />
                </div>
                <div>{ format!("Weight: {} (Capacity: {})", weight, capacity) }</div>
                <div>{ format!("Initiative Penalty: {}", penalty) }</div>
            </div>
        }
    }
}

