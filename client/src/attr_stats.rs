use yew::prelude::*;

pub struct AttrStats {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: &'static str,
    pub stats: (i32, i32, i32),
}

impl Component for AttrStats {
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
        let (base, modifier, bonus) = self.props.stats;
        let (modified, bonus) = (base + modifier, bonus);
        let style = if bonus < 0 { "color:red" } else { "" };
        html! {
            <div class="attr-stats">
                <div class="a-title">{ self.props.title }</div>
                <div class="value">{ modified }</div>
                <div class="bonus" style=style>{ bonus }</div>
            </div>
        }
    }
}

