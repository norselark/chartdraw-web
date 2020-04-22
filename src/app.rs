use std::str::FromStr;
use yew::prelude::*;

use crate::components::{
    BottomBar, CanvasArea, CycleSelect, HarmonicSelect, ListView, TextInput, TopBar,
};

pub struct App {
    link: ComponentLink<Self>,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Positions(pub [f64; 13]);

impl Default for Positions {
    fn default() -> Self {
        Self([0.; 13])
    }
}

impl Positions {
    pub fn sun(&self) -> f64 {
        self.0[0]
    }

    pub fn moon(&self) -> f64 {
        self.0[1]
    }

    pub fn descendant(&self) -> f64 {
        self.0[11]
    }

    pub fn ascendant(&self) -> f64 {
        self.0[12]
    }

    pub fn planets(&self) -> &[f64] {
        &self.0[0..11]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HarmonicCycle {
    Base,
    Cycle(u8),
    Harmonic(u16),
}

impl Default for HarmonicCycle {
    fn default() -> Self {
        Self::Base
    }
}

pub enum Msg {
    Noop,
    CycleChange(u8),
    HarmonicChange(u16),
    NewPositions(Positions),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            harmonic_cycle: Default::default(),
            positions: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CycleChange(1) => self.harmonic_cycle = HarmonicCycle::Base,
            Msg::HarmonicChange(1) => self.harmonic_cycle = HarmonicCycle::Base,
            Msg::CycleChange(cycle) => self.harmonic_cycle = HarmonicCycle::Cycle(cycle),
            Msg::HarmonicChange(harmonic) => {
                self.harmonic_cycle = HarmonicCycle::Harmonic(harmonic)
            }
            Msg::Noop => return false,
            Msg::NewPositions(positions) => self.positions = positions,
        }
        true
    }

    fn view(&self) -> Html {
        let on_harmonic_change = self
            .link
            .callback(|h| Msg::HarmonicChange(h));
        let on_cycle_change = self
            .link
            .callback(|c| Msg::CycleChange(c));
        let on_positions_change = self.link.callback(|pos: Positions| Msg::NewPositions(pos));

        let (harmonic, cycle) = match self.harmonic_cycle {
            HarmonicCycle::Base => (1, 1),
            HarmonicCycle::Harmonic(harm) => (harm, 1),
            HarmonicCycle::Cycle(cycl) => (1, cycl),
        };

        let drawing_positions = harmonics(&self.positions, harmonic);

        html! {
            <div class="app_container">
                <div class="left_frame">
                    <TopBar />
                    <CanvasArea harmonic_cycle=&self.harmonic_cycle positions=&drawing_positions />
                    <BottomBar harmonic_cycle=&self.harmonic_cycle />
                </div>
                <div class="right_frame">
                    <HarmonicSelect harmonic=harmonic on_change=on_harmonic_change />
                    <CycleSelect cycle=cycle on_change=on_cycle_change />
                    <ListView positions=&self.positions />
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
    let mut new_positions: [f64; 13] = [0.; 13];
    for (i, pos) in positions.0.iter().enumerate() {
        new_positions[i] = (pos * harmonic as f64) % 360.
    }
    Positions(new_positions)
}
