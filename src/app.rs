use std::str::FromStr;
use yew::prelude::*;

use crate::components::{BottomBar, CanvasArea, CycleSelect, HarmonicSelect, ListView, TopBar};
use crate::input;

pub struct App {
    link: ComponentLink<Self>,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Positions(pub [f64; 13]);

impl Default for Positions {
    fn default() -> Self {
        Self([
            292.24, 242.66, 271.75, 293.01, 231.33, 228.77, 272.72, 24.61, 342.18, 289.17, 135.16,
            306.4, 87.37,
        ])
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
        self.harmonic_cycle = match msg {
            Msg::CycleChange(1) => HarmonicCycle::Base,
            Msg::HarmonicChange(1) => HarmonicCycle::Base,
            Msg::CycleChange(cycle) => HarmonicCycle::Cycle(cycle),
            Msg::HarmonicChange(harmonic) => HarmonicCycle::Harmonic(harmonic),
            Msg::Noop => return false,
        };
        true
    }

    fn view(&self) -> Html {
        let on_harmonic_change = self
            .link
            .callback(|cd| Msg::HarmonicChange(try_from_change_data::<u16>(cd).unwrap()));
        let on_cycle_change = self
            .link
            .callback(|cd| Msg::CycleChange(try_from_change_data::<u8>(cd).unwrap()));

        let (harmonic, cycle) = match self.harmonic_cycle {
            HarmonicCycle::Base => (1, 1),
            HarmonicCycle::Harmonic(harm) => (harm, 1),
            HarmonicCycle::Cycle(cycl) => (1, cycl),
        };

        let input_main = self.link.callback(|_| {
            input::main();
            Msg::Noop
        });

        html! {
            <div class="app_container">
                <div class="left_frame">
                    <TopBar />
                    <CanvasArea harmonic_cycle=&self.harmonic_cycle positions=&self.positions />
                    <BottomBar harmonic_cycle=&self.harmonic_cycle />
                </div>
                <div class="right_frame">
                    <HarmonicSelect harmonic=harmonic on_change=on_harmonic_change />
                    <CycleSelect cycle=cycle on_change=on_cycle_change />
                    <h3>{ "Harmonic und Cycle" }</h3>
                    <p>{ format!("{:?}", self.harmonic_cycle ) }</p>
                    <ListView positions=&self.positions />
                </div>
                <button onclick=input_main>{ "Test" }</button>
            </div>
        }
    }
}

fn try_from_change_data<T: FromStr>(cd: ChangeData) -> Result<T, T::Err> {
    match cd {
        ChangeData::Value(val) => val.parse(),
        _ => unreachable!(),
    }
}
