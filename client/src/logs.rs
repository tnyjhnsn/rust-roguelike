use yew::prelude::*;
use super::log::*;
use super::model::log_model::*;

pub struct Logs {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub logs: MLog,
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
        let render_log = |log: &(u8, String)| {
            html! {
                <Log log=log />
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

