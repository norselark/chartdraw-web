use super::IntegerInput;
use yew::prelude::*;

pub struct HarmonicSelect {
    _link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic: u16,
    pub on_change: Callback<u16>,
}

impl Component for HarmonicSelect {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <div class="form-group">
                <label for="harmonic-select">{ "Harmonic:" }</label>
                <IntegerInput
                    id="harmonic-select"
                    value=self.props.harmonic
                    min=1
                    max=300
                    on_change=self.props.on_change.clone()
                />
            </div>
        }
    }
}
