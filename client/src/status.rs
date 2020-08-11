use yew::prelude::*;
use yew::services::ConsoleService;
use roguelike_common::*;

pub struct Status {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub status: i32
}

impl Component for Status {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Status { props }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            //ConsoleService::info("TILE CHANGE True");
            self.props = props;
            true
        } else {
            //ConsoleService::info("TILE CHANGE False");
            false
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        ConsoleService::info("STATUS UPDATE");
        false
    }

    fn view(&self) -> Html {
        //ConsoleService::info("RENDER TILE");
        let mut style = String::from("not-seen");
        if self.props.status & SEEN != 0 { style = String::from("seen") };
        if self.props.status & VISIBLE != 0 { style = String::from("visible") };

        html! {
            <div class=("tile", style)></div>
        }

    }
}

