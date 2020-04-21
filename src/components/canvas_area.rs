use yew::prelude::*;

use crate::app::{HarmonicCycle, Positions};
use crate::optimize;

const ZODIAC_GLYPHS: [char; 12] = [
    '♈', '♉', '♊', '♋', '♌', '♍', '♎', '♏', '♐', '♑', '♒', '♓',
];
const PLANET_GLYPHS: [char; 11] = ['☉', '☽', '☿', '♀', '♂', '♃', '♄', '♅', '♆', '♇', '☊'];

pub struct CanvasArea {
    // link: ComponentLink<Self>,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic_cycle: HarmonicCycle,
    pub positions: Positions,
}

impl Component for CanvasArea {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            harmonic_cycle: props.harmonic_cycle,
            positions: props.positions,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.harmonic_cycle = props.harmonic_cycle;
        self.positions = props.positions;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cycleoffset = 30.
            * match self.harmonic_cycle {
                HarmonicCycle::Cycle(n) => n as f64 - 1.,
                _ => 0.,
            };
        let start_of_zodiac = self.positions.ascendant() - cycleoffset;

        html! {
            <svg
                version="1.1" baseProfile="full"
                width="512" height="512"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="-1 -1 2 2" stroke-width="0.01"
                text-anchor="middle" dominant-baseline="central"
                font-size="12"
            >
                <rect x="-1" y="-1" width="2" height="2" fill="black" />
                <circle r="0.863" stroke="white" fill="#55ffff" />
                <circle r="0.703" stroke="white" fill="#5555ff" />
                { house_sectors() }
                // Blue semicircle under horizon
                <g transform=format!("rotate({})", cycleoffset)>
                    <path d="M -0.703 0 A 0.5 0.5 0 0 0 0.703 0" fill="#0000aa" stroke="white" />
                </g>
                <g transform=format!("rotate({})", -start_of_zodiac)>
                    { five_deg_lines() }
                    { zodiac_sectors(start_of_zodiac) }
                    { planet_markers(self.positions.planets(), start_of_zodiac) }
                </g>
                <circle r="0.883" stroke="white" fill="transparent" />
                <g stroke="#00aaaa" fill="transparent">
                    <circle r="0.930" />
                    <circle r="1.043" />
                </g>
                // Ascendant arrow
                <g transform=format!("rotate({})", cycleoffset) stroke="white">
                    <path d="M 0.703 0 H 0.219 M -0.219 0 H -0.703 M -0.863 0 H -0.9" stroke="white" />
                    <path d="M -0.9 0.01 h -0.02 l -0.03 -0.01 l 0.03 -0.01 h 0.02 z " fill="white" />
                </g>
                // Descendant arrow
                <g transform=format!("rotate({})", cycleoffset + self.positions.descendant()) stroke="white">
                    <path d="M 0.703 0 H 0.219 M -0.219 0 H -0.703 M -0.863 0 H -0.9" stroke="white" />
                    <path d="M -0.9 0.01 h -0.02 l -0.03 -0.01 l 0.03 -0.01 h 0.02 z " stroke="white" fill="black" />
                </g>
                // Centre disk
                <circle r="0.219" stroke="black" fill="#55ffff" />
            </svg>
        }
    }
}

fn house_sectors() -> Html {
    html! {
        <g stroke="white">
            {
                for (0..12).map(|offset| {
                    let rotation = format!("rotate({})", 30 * offset);
                    html!{<path d="M 0.703 0 L 0.863 0" transform=rotation />}
                })
            }
        </g>
    }
}

fn five_deg_lines() -> Html {
    html! {
        <g stroke="white">
            { for (0..72).map(|offset| {
                let rotation = format!("rotate({})", 5 * offset);
                html!{
                    <path d="M 0.863 0 L 0.883 0" transform=rotation />
                }
            }) }
        </g>
    }
}

/// start_of_zodiac is passed in order to flip the glyphs right side up
fn zodiac_sectors(start_of_zodiac: f64) -> Html {
    let sector = |offset| {
        let angle = (30 * offset) as f64;
        let rotation = format!("rotate({})", -angle);
        let text_trans = format!(
            "rotate(-15) translate(0.98, 0) rotate({}) scale(0.008)",
            angle + 15. + start_of_zodiac
        );
        let glyph = ZODIAC_GLYPHS[offset];
        html! {
            <g transform=rotation>
                <path d="M 0.930 0 L 1.043 0" stroke="#00aaaa" />
                <text fill="#00aaaa" transform=text_trans>
                    { glyph }
                </text>
            </g>
        }
    };
    html! {
        <g>{ for (0..12).map(sector) }</g>
    }
}

fn planet_markers(positions: &[f64], start_of_zodiac: f64) -> Html {
    let optimized_position = optimize::optimize(positions);
    html! {
        <g>
            { for positions.iter().enumerate().map(|(i, a)| {
                let delta = optimized_position[i] - a;
                let text_trans = format!("rotate({}) translate(0.775, 0) rotate({}) scale(0.008)", -delta, a + delta + start_of_zodiac);
                let glyph = PLANET_GLYPHS[i];
                html! {
                    <g transform=format!("rotate({})", -a) stroke-width="0.005">
                        <circle cx="0.684" r="0.012" stroke="white" fill="transparent" />
                        <circle cx="0.902" r="0.015" fill="white" />
                        <text fill="black" transform=text_trans>
                            { glyph }
                        </text>
                    </g>
                }
            } ) }
        </g>
    }
}
