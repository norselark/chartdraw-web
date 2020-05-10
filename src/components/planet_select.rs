use crate::app::try_from_change_data;
use std::num::ParseIntError;
use yew::prelude::*;

const min_planets: u8 = 1;
const max_planets: u8 = 9;

pub struct PlanetSelect {
    link: ComponentLink<Self>,
    planets: u16,
    on_change: Callback<u16>,
    error: Option<ParseIntError>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub planets: u8,
    pub on_change: Callback<u16>,
}

pub struct Msg(ChangeData);

impl Component for PlanetSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            planets: props.planets,
            on_change: props.on_change,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg(cd) => match try_from_change_data::<u16>(cd) {
                Ok(v) => {
                    self.error = None;
                    self.on_change.emit(v);
                }
                Err(detail) => self.error = detail.into(),
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.planets = props.planets;
        true
    }

    fn view(&self) -> Html {
        let maybe_error = match &self.error {
            Some(err) => html! { <div class="alert alert-warning">{ err.to_string() }</div> },
            None => html! {},
        };
        html! {
            <div class="form-group">
                <label for="planet-select">{ "Number of planets:" }</label>
                <input
                    id="planet-select"
                    class="form-control"
                    type="number"
                    value=self.planets
                    min=min_planets
                    max=max_planets
                    onchange=self.link.callback(Msg)
                />
                { maybe_error }
            </div>
        }
    }
}
