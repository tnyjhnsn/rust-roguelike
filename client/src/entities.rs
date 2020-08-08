use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;

pub struct Entities {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub entities: EntityPositions,
}

impl Component for Entities {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Entities { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props || self.props.entities.len() != 0 {
            ConsoleService::info("ENTITY CHANGE True");
            self.props = props;
            true
        } else {
            ConsoleService::info("ENTITY CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("ENTITY UPDATE");
        false
    }

    fn view(&self) -> Html {
        ConsoleService::info("RENDER ENTITY");
        let render_tile = |i| {
            let mut s = String::new();
            for (a, _b) in self.props.entities.iter() {
                match *a == i {
                    true => {
                        s = String::from("player-m");
                        break;
                    },
                    _ => s = String::from(" "),
                };
            };
            html! {
                <div class=("tile", s)></div>
            }

        };
        html! {
            <div class="entities">
                { for (0..1200).map(render_tile) }
            </div>
        }
    }
}


