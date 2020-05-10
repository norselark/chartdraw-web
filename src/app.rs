use std::str::FromStr;
use yew::prelude::*;

use crate::components::{
    ListView, TextInput,
    Drawing,
};

pub struct App {
    link: ComponentLink<Self>,
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


pub enum Msg {
    #[allow(unused)]
    Noop,
    NewPositions(Positions),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            positions: Positions::default(),
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Noop => return false,
            Msg::NewPositions(positions) => self.positions = positions,
        }
        true
    }

    fn view(&self) -> Html {
        let on_positions_change = self.link.callback(Msg::NewPositions);

        html! {
            <div class="container">
                <div class="row">
                    <Drawing positions=&self.positions />
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

pub fn harmonics(positions: &Positions, harmonic: u16) -> Positions {
    let mut new_positions: [f32; 13] = [0.; 13];
    for (i, pos) in positions.0.iter().enumerate() {
        new_positions[i] = (pos * f32::from(harmonic)) % 360.
    }
    Positions(new_positions)
}
