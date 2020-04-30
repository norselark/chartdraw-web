use crate::app::Positions;
use crate::input;
use yew::prelude::*;

pub struct TextInput {
    link: ComponentLink<Self>,
    text: String,
    on_change: Callback<Positions>,
    error: Option<input::Error>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Callback<Positions>,
}

pub enum Msg {
    TextInput(String),
    Clicked,
    FillDefault,
}

impl Component for TextInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::new(),
            on_change: props.on_change,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TextInput(text) => self.text = text,
            Msg::Clicked => match input::parse_zet9(&self.text) {
                Ok(positions) => {
                    self.error = None;
                    self.on_change.emit(positions);
                }
                Err(detail) => {
                    web_sys::console::error_1(&format!("{:?}", detail).into());
                    self.error = Some(detail);
                }
            },
            Msg::FillDefault => self.text = input::SAMPLE.to_string(),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_text_input = self.link.callback(|e: InputData| Msg::TextInput(e.value));
        let onclick = self.link.callback(|_| Msg::Clicked);
        let fill_default = self.link.callback(|_| Msg::FillDefault);

        let label = html! {
            <label>
                { "Paste ZET9 output here and click submit. Click " }
                <em>{ "Insert sample" }</em>
                { " to see an example." }
            </label>
        };
        let maybe_error = match &self.error {
            Some(err) => html! { <div class="alert alert-warning">{ format!("{:?}", err) }</div> },
            None => html! {},
        };

        html! {
            <div>
                <form>
                    <div class="form-group">
                        { label }
                        { maybe_error }
                        <textarea class="form-control text-monospace" rows=10 value=self.text, oninput=on_text_input />
                    </div>
                </form>
                <div class="btn-group" role="group">
                    <button class="btn" onclick=onclick>{ "Submit" }</button>
                    <button class="btn" onclick=fill_default>{ "Insert sample" }</button>
                </div>
            </div>
        }
    }
}
