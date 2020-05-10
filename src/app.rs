use std::str::FromStr;
use yew::prelude::*;

use crate::components::{
    BottomBar, CanvasArea, CycleSelect, HarmonicSelect, ListView, PlanetSelect, TextInput, TopBar,
};

pub struct App {
    link: ComponentLink<Self>,
    aspect: bool,
    harmonic_cycle: HarmonicCycle,
    planets: u16,
    positions: Positions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Positions(pub [f32; 13]);

impl Default for Positions {
    fn default() -> Self {
        Self([0.; 13])
    }
}

impl Positions {
    /// The position of the sun
    pub fn sun(&self) -> f32 {
        self.0[0]
    }

    /// The position of the moon
    pub fn moon(&self) -> f32 {
        self.0[1]
    }

    /// The position of the descendant
    pub fn descendant(&self) -> f32 {
        self.0[11]
    }

    /// The position of the ascendant
    pub fn ascendant(&self) -> f32 {
        self.0[12]
    }

    /// The position of the eleven planets, from the Sun up to and including the Node
    pub fn planets(&self) -> &[f32] {
        &self.0[0..11]
    }

    /// The position of the ten planets, including Pluto but not Node
    pub fn planets_without_node(&self) -> &[f32] {
        &self.0[0..10]
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

pub enum Msg {
    #[allow(unused)]
    Noop,
    ToggleAspect,
    CycleChange(u16),
    HarmonicChange(u16),
    PlanetsChange(u16),
    NewPositions(Positions),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            aspect: false,
            harmonic_cycle: HarmonicCycle::default(),
            positions: Positions::default(),
            planets: 9,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
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
            Msg::NewPositions(positions) => self.positions = positions,
            Msg::PlanetsChange(planets) => self.planets = planets,
        }
        true
    }

    fn view(&self) -> Html {
        let on_harmonic_change = self.link.callback(Msg::HarmonicChange);
        let on_cycle_change = self.link.callback(Msg::CycleChange);
        let on_positions_change = self.link.callback(Msg::NewPositions);
        let on_aspect_toggle = self.link.callback(|_| Msg::ToggleAspect);
        let on_planets_change = self.link.callback(Msg::PlanetsChange);

        let (harmonic, cycle) = match self.harmonic_cycle {
            HarmonicCycle::Base => (1, 0),
            HarmonicCycle::Harmonic(harm) => (harm, 0),
            HarmonicCycle::Cycle(cycl) => (1, cycl),
        };

        let drawing_positions = harmonics(&self.positions, harmonic);

        html! {
            <div class="container">
                <div class="row">
                    <div class="col-md-5">
                        <TopBar />
                        <CanvasArea harmonic_cycle=&self.harmonic_cycle positions=&drawing_positions aspect=self.aspect />
                        <BottomBar harmonic_cycle=&self.harmonic_cycle />
                    </div>
                    <div class="col">
                        <h4>{ "Drawing controls" }</h4>
                        <form>
                            <div class="form-check">
                                <input id="aspect-toggle" class="form-check-input" type="checkbox"
                                    checked=self.aspect onchange=on_aspect_toggle />
                                <label for="aspect-toggle" class="form-check-label">{ "Show aspects" }</label>
                            </div>
                            <HarmonicSelect harmonic=harmonic on_change=on_harmonic_change />
                            <CycleSelect cycle=cycle on_change=on_cycle_change />
                            <PlanetSelect planets=self.planets on_change=on_planets_change />
                        </form>
                    </div>
                    <div class="col">
                        <h4>{ "Numeric positions" }</h4>
                        <ListView positions=&self.positions />
                    </div>
                </div>
                <TextInput on_change=on_positions_change />
            </div>
        }
    }
}

pub fn try_from_change_data<T: FromStr>(cd: ChangeData) -> Result<T, T::Err> {
    match cd {
        ChangeData::Value(val) => val.parse(),
        _ => unreachable!(),
    }
}

fn harmonics(positions: &Positions, harmonic: u16) -> Positions {
    let mut new_positions: [f32; 13] = [0.; 13];
    for (i, pos) in positions.0.iter().enumerate() {
        new_positions[i] = (pos * f32::from(harmonic)) % 360.
    }
    Positions(new_positions)
}
