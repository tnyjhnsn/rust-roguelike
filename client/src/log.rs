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
            2 => format!("There is no way down from here"),
            _ => format!("Unknown System Message"),
        }
    }

    fn get_attack_msg(&self, attacker: i32, target: i32, damage: i32) -> Html {
        let attacker_name = self.props.dict.get_name(attacker);
        let attacker_css = self.props.dict.get_css(attacker);
        let target_name = self.props.dict.get_name(target);
        let content = format!("The {} attacks the {} causing {} damage", attacker_name, target_name, damage);
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
        format!("The {} is dead", deceased_name)
    }

    fn get_destroyed_msg(&self, destroyed: i32) -> String {
        let destroyed_name = self.props.dict.get_name(destroyed);
        format!("The {} has been destroyed", destroyed_name)
    }

    fn get_collect_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} picks up the {}", entity_name, item_name)
    }

    fn get_drop_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} drops the {}", entity_name, item_name)
    }

    fn get_unequips_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} unequips the {}", entity_name, item_name)
    }

    fn get_equips_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} equips the {}", entity_name, item_name)
    }

    fn get_removes_msg(&self, entity: i32, item: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} removes the {}", entity_name, item_name)
    }

    fn get_drink_msg(&self, entity: i32, item: i32, amount: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        format!("The {} drinks the {} for {} healing", entity_name, item_name, amount)
    }

    fn get_use_item_msg(&self, entity: i32, item: i32, target: i32, amount: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        let target_name = self.props.dict.get_name(target);
        format!("The {} uses the {} on the {} causing {} damage", entity_name, item_name, target_name, amount)
    }

    fn get_confused_msg(&self, entity: i32, item: i32, target: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let item_name = self.props.dict.get_name(item);
        let target_name = self.props.dict.get_name(target);
        format!("The {} uses the {} on the {}", entity_name, item_name, target_name)
    }

    fn get_trap_msg(&self, entity: i32, trap: i32, amount: i32) -> String {
        let entity_name = self.props.dict.get_name(entity);
        let trap_name = self.props.dict.get_name(trap);
        format!("The {} falls into the {} suffering {} damage", entity_name, trap_name, amount)
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
        let render_log = |log: &Vec<i32>| {
            html! {
                <li>
                    <div class="flex-wrap">
                    {
                        match log[0] {
                            0 => html! { self.get_system_msg(log[1]) },
                            1 => self.get_attack_msg(log[1], log[2], log[3]),
                            2 => html! { self.get_dead_msg(log[1]) },
                            3 => html! { self.get_collect_msg(log[1], log[2]) },
                            4 => html! { self.get_drop_msg(log[1], log[2]) },
                            5 => html! { self.get_drink_msg(log[1], log[2], log[3]) },
                            6 => html! { self.get_use_item_msg(log[1], log[2], log[3], log[4]) },
                            7 => html! { self.get_destroyed_msg(log[1]) },
                            8 => html! { self.get_confused_msg(log[1], log[2], log[3]) },
                            9 => html! { self.get_unequips_msg(log[1], log[2]) },
                            10 => html! { self.get_equips_msg(log[1], log[2]) },
                            11 => html! { self.get_removes_msg(log[1], log[2]) },
                            12 => html! { self.get_trap_msg(log[1], log[2], log[3]) },
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
                    .map(render_log)}
            </ul>
        }
    }
}

