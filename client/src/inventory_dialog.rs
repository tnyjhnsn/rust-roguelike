use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::utils::document;
use web_sys::{HtmlElement};

pub struct InventoryDialog {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub show: bool,
    pub onkeydown_signal: Callback<KeyboardEvent>,
}

pub enum Msg {
    Pressed(KeyboardEvent),
}

impl Component for InventoryDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Pressed(e) => {
                self.props.onkeydown_signal.emit(e);
            }
        }
        true
    }

    fn rendered(&mut self, _: bool) {
        if self.props.show {
            document()
                .get_elements_by_class_name("modal")
                .get_with_index(0)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .unwrap()
                .focus()
                .unwrap();
        }
    }

    fn view(&self) -> Html {
        let inv_style = if self.props.show { "display: block;" } else { "display: none;" };
        html! {
            <div
                class="modal"
                style=inv_style
                tabindex="-1"
                onkeydown=self.link.callback(Msg::Pressed)
            >
                <div class="modal-content">
                { "Here's a modal!!" }
                </div>
            </div>
        }
    }
}

