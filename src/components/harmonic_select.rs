use yew::prelude::*;

pub struct HarmonicSelect {
    link: ComponentLink<Self>,
    harmonic: u16,
    on_change: Callback<ChangeData>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic: u16,
    pub on_change: Callback<ChangeData>,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.on_change.emit(msg.0);
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
            </label>
        }
    }
}
