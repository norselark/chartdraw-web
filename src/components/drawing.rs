use yew::prelude::*;

use super::{BottomBar, CycleSelect, HarmonicSelect, PlanetSelect, SvgChart, TopBar};
use crate::app::{harmonics, Positions};

pub struct Drawing {
    link: ComponentLink<Self>,
    props: Props,
    harmonic_cycle: HarmonicCycle,
    aspect: bool,
    planets: u16,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub positions: Positions,
}

pub enum Msg {
    #[allow(unused)]
    Noop,
    ToggleAspect,
    CycleChange(u16),
    HarmonicChange(u16),
    PlanetsChange(u16),
}

impl Component for Drawing {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            harmonic_cycle: HarmonicCycle::default(),
            aspect: false,
            planets: 9,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CycleChange(0) | Msg::HarmonicChange(1) => {
                self.harmonic_cycle = HarmonicCycle::Base
            }
            Msg::CycleChange(cycle) => self.harmonic_cycle = HarmonicCycle::Cycle(cycle),
            Msg::HarmonicChange(harmonic) => {
                self.harmonic_cycle = HarmonicCycle::Harmonic(harmonic)
            }
            Msg::Noop => return false,
            Msg::ToggleAspect => self.aspect = !self.aspect,
            Msg::PlanetsChange(planets) => self.planets = planets,
        }
        true
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
        let on_harmonic_change = self.link.callback(Msg::HarmonicChange);
        let on_cycle_change = self.link.callback(Msg::CycleChange);
        let on_aspect_toggle = self.link.callback(|_| Msg::ToggleAspect);
        let on_planets_change = self.link.callback(Msg::PlanetsChange);

        let (harmonic, cycle) = match self.harmonic_cycle {
            HarmonicCycle::Base => (1, 0),
            HarmonicCycle::Harmonic(h) => (h, 0),
            HarmonicCycle::Cycle(c) => (1, c),
        };

        let drawing_positions = harmonics(&self.props.positions, harmonic);

        html! {
            <>
            <div class="col-md-5">
                <TopBar />
                <SvgChart
                    harmonic_cycle=&self.harmonic_cycle
                    positions=&drawing_positions
                    aspect=self.aspect
                />
                <BottomBar harmonic_cycle=&self.harmonic_cycle />
            </div>
            <div class="col">
                <h4>{ "Drawing controls" }</h4>
                <form>
                    <div class="form-check">
                        <input
                            id="aspect-toggle"
                            class="form-check-input"
                            type="checkbox"
                            checked=self.aspect
                            onchange=on_aspect_toggle
                        />
                        <label for="aspect-toggle" class="form-check-label">
                            { "Show aspects" }
                        </label>
                    </div>
                    <HarmonicSelect harmonic=harmonic on_change=on_harmonic_change />
                    <CycleSelect cycle=cycle on_change=on_cycle_change />
                    <PlanetSelect planets=self.planets on_change=on_planets_change />
                </form>
            </div>
            </>
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HarmonicCycle {
    Base,
    Cycle(u16),
    Harmonic(u16),
}

impl Default for HarmonicCycle {
    fn default() -> Self {
        Self::Base
    }
}
