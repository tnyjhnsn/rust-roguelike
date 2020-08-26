use yew::prelude::*;
use chrono::prelude::*;

pub struct Log {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub log: (DateTime<Local>, Vec<Vec<i32>>),
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
        html! {
            <div>{ format!("{:?} {:?}", &self.props.log.0, &self.props.log.1) }</div>
        }
    }
}

