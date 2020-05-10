use super::IntegerInput;
use yew::prelude::*;

const MIN_PLANETS: u16 = 1;
const MAX_PLANETS: u16 = 9;

pub struct PlanetSelect {
    _link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub planets: u16,
    pub on_change: Callback<u16>,
}

pub struct Msg(u16);

impl Component for PlanetSelect {
    type Message = Msg;
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
                <label for="planet-select">{ "Number of planets" }</label>
                <IntegerInput
                    id="planet-select"
                    value=self.props.planets
                    min=MIN_PLANETS
                    max=MAX_PLANETS
                    on_change=self.props.on_change.clone()
                />
            </div>
        }
    }
}
