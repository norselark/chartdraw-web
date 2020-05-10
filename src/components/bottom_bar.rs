use super::drawing::HarmonicCycle;
use itertools::Itertools;
use yew::prelude::*;

static BASE_TEXT: &str = "2-D Radix\nHorizon view\nOrigo: Tropos";
static CYCLE_TEXT: &str = "2-D Turned\nDerived houses\nRadix Quadrants";

pub struct BottomBar {
    harmonic_cycle: HarmonicCycle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic_cycle: HarmonicCycle,
}

impl Component for BottomBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            harmonic_cycle: props.harmonic_cycle,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.harmonic_cycle = props.harmonic_cycle;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let text = match self.harmonic_cycle {
            HarmonicCycle::Cycle(_) => CYCLE_TEXT,
            _ => BASE_TEXT,
        };
        html! {
            <div>{
                for text.lines().map(|s| html! { s }).intersperse(html! { <br /> })
            }</div>
        }
    }
}
