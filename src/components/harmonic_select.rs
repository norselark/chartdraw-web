use yew::prelude::*;
use crate::app::try_from_change_data;
use std::num::ParseIntError;

pub struct HarmonicSelect {
    link: ComponentLink<Self>,
    harmonic: u16,
    on_change: Callback<u16>,
    error: Option<ParseIntError>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic: u16,
    pub on_change: Callback<u16>,
}

pub struct Msg(ChangeData);

impl Component for HarmonicSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            harmonic: props.harmonic,
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
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.harmonic = props.harmonic;
        true
    }

    fn view(&self) -> Html {
        html! {
            <label>
                { "Harmonic:" }
                <input type="number" value=self.harmonic min=1 max=300
                    onchange=self.link.callback(|cd| Msg(cd)) />
                { match &self.error {
                    Some(err) => html! { <p class="error">{ err.to_string() }</p> },
                    None => html! {},
                } }
            </label>
        }
    }
}