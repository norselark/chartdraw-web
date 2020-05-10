use crate::app::try_from_change_data;
use std::num::ParseIntError;
use yew::prelude::*;

pub struct IntegerInput {
    link: ComponentLink<Self>,
    props: Props,
    error: Option<ParseIntError>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: u16,
    pub min: u16,
    pub max: u16,
    pub id: String,
    pub on_change: Callback<u16>,
}

pub struct Msg(ChangeData);

impl Component for IntegerInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg(cd) => match try_from_change_data::<u16>(cd) {
                Ok(v) => {
                    self.error = None;
                    self.props.on_change.emit(v);
                }
                Err(detail) => self.error = detail.into(),
            },
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let maybe_error = match &self.error {
            Some(err) => html! { <div class="alert alert-warning">{ err.to_string() }</div> },
            None => html! {},
        };
        html! {
            <>
            <input
                id=self.props.id
                class="form-control"
                type="number"
                value=self.props.value
                min=1
                max=300
                onchange=self.link.callback(Msg)
            />
            { maybe_error }
            </>
        }
    }
}
