use yew::prelude::*;
use chrono::prelude::*;
use super::model::dictionary::*;

pub struct Log {
    props: Props,
}

impl Log {
    fn get_system_msg(&self) -> String {
        format!("Hello Rogue!")
    }

    fn get_attack_msg(&self) -> String {
        let msg = &self.props.log.1[0]; 
        let attacker = self.props.dict.get_name(msg[1]);
        let target = self.props.dict.get_name(msg[2]);
        format!("{} attacks {} for {} damage", attacker, target, msg[3])
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub log: (DateTime<Local>, Vec<Vec<i32>>),
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
        let s = match &self.props.log.1[0][0] {
            0 => self.get_system_msg(),
            1 => self.get_attack_msg(),
            _ => format!("Something else"),
        };
        html! {
            <div>{ s }</div>
        }
    }
}

