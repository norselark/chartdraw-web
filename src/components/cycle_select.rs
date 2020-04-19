use yew::prelude::*;

pub struct CycleSelect {
    link: ComponentLink<Self>,
    cycle: u8,
    on_change: Callback<ChangeData>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub cycle: u8,
    pub on_change: Callback<ChangeData>,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.on_change.emit(msg.0);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cycle = props.cycle;
        true
    }

    fn view(&self) -> Html {
        html! {
            <label>
                { "Cycle" }
                <input type="number" value=self.cycle min=1 max=12
                    onchange=self.link.callback(|cd| Msg(cd)) />
            </label>
        }
    }
}
