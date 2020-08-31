use yew::prelude::*;
use super::model::dictionary::*;

pub struct Log {
    props: Props,
}

impl Log {
    fn get_system_msg(&self, msg: i32) -> String {
        match msg {
            0 => format!("Hello Rogue!"),
            1 => format!("There is nothing to pick up here"),
            _ => format!("Unknown System Message"),
        }
    }

    fn get_attack_msg(&self, attacker: i32, target: i32, damage: i32) -> Html {
        let attacker_name = self.props.dict.get_name(attacker);
        let attacker_css = self.props.dict.get_css(attacker);
        let target_name = self.props.dict.get_name(target);
        let content = format!("{} attacks {} for {} damage", attacker_name, target_name, damage);
        html! {
            <>
                <div class="tile-box">
                    <div class=("tile", attacker_css)></div>
                </div>
                <div class="content">{ content }</div>
            </>
        }
    }

    fn get_dead_msg(&self, deceased: i32) -> String {
        let deceased_name = self.props.dict.get_name(deceased);
        format!("{} is dead", deceased_name)
    }

    fn get_collect_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("{} picks up the {}", entity_name, item_name)
    }

    fn get_drop_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("{} drops the {}", entity_name, item_name)
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
            html! {
                <li>
                    <div class="flex-wrap">
                    {
                        match msg[0] {
                            0 => html! { self.get_system_msg(msg[1]) },
                            1 => self.get_attack_msg(msg[1], msg[2], msg[3]),
                            2 => html! { self.get_dead_msg(msg[1]) },
                            3 => html! { self.get_collect_msg(msg[1], msg[2]) },
                            4 => html! { self.get_drop_msg(msg[1], msg[2]) },
                            _ => html! { "Unknown log message" },
                        }
                    }
                    </div>
                </li>
            }
        };
        html! {
            <ul>
                { for self.props.log
                    .iter()
                    .map( render_log )}
            </ul>
        }
    }
}

