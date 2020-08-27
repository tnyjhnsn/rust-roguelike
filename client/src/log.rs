use yew::prelude::*;
use super::model::dictionary::*;

pub struct Log {
    props: Props,
}

impl Log {
    fn get_system_msg(&self) -> String {
        format!("Hello Rogue!")
    }

    fn get_attack_msg(&self, attacker: i32, target: i32, damage: i32) -> String {
        let attacker_name = self.props.dict.get_name(attacker);
        let target_name = self.props.dict.get_name(target);
        format!("{} attacks {} for {} damage", attacker_name, target_name, damage)
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub log: Vec<Vec<i32>>,
    pub dict: Dictionary,
}

impl Component for Log {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Log { props }
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
        let render_log = |_l| {
            let msg = &self.props.log[0];
            match msg[0] {
                0 => html! { self.get_system_msg() },
                1 => html! { self.get_attack_msg(msg[1], msg[2], msg[3]) },
                _ => html! { "Something else" },
            }
        };
        html! {
            { for self.props.log
                .iter()
                .map( render_log )}
        }
    }
}

