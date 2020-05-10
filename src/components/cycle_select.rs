use super::IntegerInput;
use yew::prelude::*;

pub struct CycleSelect {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cycle: u16,
    pub on_change: Callback<u16>,
}

pub struct Msg(u16);

impl Component for CycleSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg(v) => {
                self.props.on_change.emit((v + 11) % 12);
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let on_change = self.link.callback(Msg);
        html! {
            <div class="form-group">
                <label for="cycle-select">{ "Turned axis" }</label>
                <IntegerInput
                    id="cycle-select"
                    value=self.props.cycle + 1
                    min=0
                    max=13
                    on_change=on_change
                />
            </div>
        }
    }
}
