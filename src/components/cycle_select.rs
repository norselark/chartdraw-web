use crate::app::try_from_change_data;
use std::num::ParseIntError;
use yew::prelude::*;

pub struct CycleSelect {
    link: ComponentLink<Self>,
    cycle: u8,
    on_change: Callback<u8>,
    error: Option<ParseIntError>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cycle: u8,
    pub on_change: Callback<u8>,
}

pub struct Msg(ChangeData);

impl Component for CycleSelect {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            cycle: props.cycle,
            on_change: props.on_change,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg(cd) => match try_from_change_data::<u8>(cd) {
                Ok(v) => {
                    self.error = None;
                    self.on_change.emit((v + 11) % 12);
                }
                Err(detail) => self.error = detail.into(),
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cycle = props.cycle;
        true
    }

    fn view(&self) -> Html {
        html! {
            <label>
                { "Turned axis:" }
                <input type="number" value=self.cycle + 1 min=0 max=13
                    onchange=self.link.callback(Msg) />
                { match &self.error {
                    Some(err) => html! { <p class="error">{ err.to_string() }</p> },
                    None => html! {},
                } }
            </label>
        }
    }
}
