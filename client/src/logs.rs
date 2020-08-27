use yew::prelude::*;
use chrono::prelude::*;
use super::log::*;
use super::model::log_model::*;
use super::model::dictionary::*;

pub struct Logs {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub logs: MLog,
    pub dict: Dictionary,
}

impl Component for Logs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Logs { props }
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
        let render_log = |log: &(DateTime<Local>, Vec<Vec<i32>>)| {
            html! {
                <Log log=log dict=&self.props.dict />
            }
        };
        html! {
            <div class="log">
                { for self.props.logs.logs
                    .iter()
                    .rev()
                    .take(5)
                    .map(render_log) }
            </div>
        }
    }
}

