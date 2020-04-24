use itertools::Itertools;
use yew::prelude::*;

use crate::app::{HarmonicCycle, Positions};
use crate::aspect;
use crate::optimize;

const ZODIAC_GLYPHS: [char; 12] = [
    '♈', '♉', '♊', '♋', '♌', '♍', '♎', '♏', '♐', '♑', '♒', '♓',
];
const PLANET_GLYPHS: [char; 11] = ['☉', '☽', '☿', '♀', '♂', '♃', '♄', '♅', '♆', '♇', '☊'];

pub struct CanvasArea {
    // link: ComponentLink<Self>,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
    zodiac_start: f64,
    cycle_offset: f64,
    aspect: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic_cycle: HarmonicCycle,
    pub positions: Positions,
    pub aspect: bool,
}

impl Component for CanvasArea {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let zodiac_start = (props.positions.ascendant() - 180.) % 360.;
        let cycle_offset = 30.
            * match props.harmonic_cycle {
                HarmonicCycle::Cycle(n) => f64::from(n) - 1.,
                _ => 0.,
            };
        Self {
            // link,
            harmonic_cycle: props.harmonic_cycle,
            positions: props.positions,
            zodiac_start,
            cycle_offset,
            aspect: props.aspect,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cycle_offset = 30.
            * match props.harmonic_cycle {
                HarmonicCycle::Cycle(n) => f64::from(n) - 1.,
                _ => 0.,
            };
        self.harmonic_cycle = props.harmonic_cycle;
        self.positions = props.positions;
        self.aspect = props.aspect;
        self.zodiac_start = (self.positions.ascendant() - 180.) % 360.;
        true
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <svg
                version="1.1" baseProfile="full"
                width="512" height="512"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="-1 -1 2 2" stroke-width="0.006"
                text-anchor="middle" dominant-baseline="central"
                font-size="12"
            >
                <rect x="-1" y="-1" width="2" height="2" fill="black" />
                <circle r="0.863" stroke="white" fill="#55ffff" />
                <circle r="0.703" stroke="white" fill="#5555ff" />
                { house_sectors() }
                // Blue semicircle under horizon
                <g transform=format!("rotate({})", self.cycle_offset)>
                    <path d="M -0.703 0 A 0.703 0.703 0 0 0 0.703 0"
                        fill="#0000aa" stroke="white" />
                </g>
                <g transform=format!("rotate({})", self.zodiac_start + self.cycle_offset)>
                    { five_deg_lines() }
                    { self.zodiac_sectors() }
                    { self.planet_markers() }
                </g>
                <circle r="0.883" stroke="white" fill="transparent" />
                <g stroke="#00aaaa" fill="transparent">
                    <circle r="0.930" />
                    <circle r="1.043" />
                </g>
                // Ascendant arrow
                <g transform=format!("rotate({})", self.cycle_offset) stroke="white">
                    <path d="M 0.703 0 H 0.219 M -0.219 0 H -0.703 M -0.863 0 H -0.9"
                        stroke="white" />
                    <path d="M -0.9 0.01 h -0.02 l -0.03 -0.01 l 0.03 -0.01 h 0.02 z "
                        fill="white" />
                </g>
                // Descendant arrow
                <g transform=format!(
                    "rotate({})",
                    self.zodiac_start - self.positions.descendant() + self.cycle_offset
                ) stroke="white">
                    <path d="M 0.703 0 H 0.219 M -0.219 0 H -0.703 M 0.863 0 H 0.9"
                        stroke="white" />
                    <path d="M 0.9 0.01 h 0.02 l 0.03 -0.01 l -0.03 -0.01 h -0.02 z "
                        stroke="white" fill="black" />
                </g>
                // Centre disk
                {
                    if let HarmonicCycle::Cycle(_) = self.harmonic_cycle {
                        self.mini_horizon()
                    } else {
                        html! { <>
                            <circle r="0.219" stroke="black" fill="#55ffff" />
                            <circle r="0.006" fill="black" />
                        </> }
                    }
                }
                {
                    if self.aspect {
                        self.aspects()
                    } else {
                        html! {}
                    }
                }
            </svg>
        }
    }
}

impl CanvasArea {
    fn mini_horizon(&self) -> Html {
        let sun_transform = format!("rotate({})", self.zodiac_start - self.positions.sun());
        let moon_transform = format!("rotate({})", self.zodiac_start - self.positions.moon());
        html! {
            <>
                <circle r="0.219" stroke="white" fill="#5555ff" />
                <path d="M -0.219 0 A 0.219 0.219 0 0 0 0.219 0" fill="#0000aa" stroke="white" />
                <path d="M -0.219 0 H 0.219 " stroke="white" />
                <path d="M -0.219 0.01 h -0.02 l -0.03 -0.01 l 0.03 -0.01 h 0.02 z " stroke="white" fill="white" />
                <g transform=format!("rotate({})", self.zodiac_start - self.positions.descendant()) stroke="white">
                    <path d="M -0.219 0 H 0.219 " />
                    <path d="M 0.219 0.01 h 0.02 l 0.03 -0.01 l -0.03 -0.01 h -0.02 z " fill="black" />
                </g>
                <circle cx=0.16 r=0.014 stroke="black" fill="yellow" transform=sun_transform />
                <circle cx=0.16 r=0.014 stroke="black" fill="lightgrey" transform=moon_transform />
                <circle r="0.03" stroke="black" fill="#55ffff" />
            </>
        }
    }

    fn aspects(&self) -> Html {
        let aspect_pairs = self
            .positions
            .planets_without_node()
            .iter()
            .tuple_combinations()
            .filter_map(|(&a, &b)| match aspect::aspect(a, b, 8.0) {
                Some(asp)
                    if matches!(
                        asp.aspect_type,
                        aspect::AspectType::Zero | aspect::AspectType::Thirty
                    ) =>
                {
                    None
                }
                Some(asp) => Some((a, b, asp)),
                None => None,
            });
        let asc_rot = format!("rotate({})", self.cycle_offset);
        let desc_rot = format!(
            "rotate({})",
            self.zodiac_start - self.positions.descendant() + self.cycle_offset
        );
        let cycle_rot = format!("rotate({})", self.cycle_offset);
        html! {
            <>
            <circle r="0.656" fill="white" />
            <g transform=format!("rotate({})", self.zodiac_start + self.cycle_offset)>
                { for aspect_pairs.map(|(a, b, aspect)| {
                    let stroke = match aspect.aspect_type {
                        aspect::AspectType::Ninety | aspect::AspectType::OneEighty => "#aa0000",
                        _ => "#00aa00",
                    };
                    let width = (0.003 * (1. + 2.2 * aspect.close)).to_string();
                    html! {
                        <path d=chord_path(0.656, a, b) stroke=stroke stroke-width=width />
                    }
                } ) }
            </g>
            <circle r="0.04" fill="#5555ff" stroke="black" />
            <path d="M -0.04 0 A 0.04 0.04 0 0 0 0.04 0" fill="#0000aa" stroke="black" transform=cycle_rot />
            <path d="M -0.04 0 H 0.04" stroke="white" transform=desc_rot />
            <path d="M -0.04 0 H 0.04" stroke="white" transform=asc_rot />
            <circle r="0.012" fill="#black" stroke="#55ffff" />
            </>
        }
    }

    /// `start_of_zodiac` is passed in order to flip the glyphs right side up
    fn zodiac_sectors(&self) -> Html {
        let sector = |offset| {
            let angle = (30 * offset) as f64;
            let rotation = format!("rotate({})", -angle);
            let text_trans = format!(
                "rotate(-15) translate(0.98, 0) rotate({}) scale(0.008)",
                angle + 15. - self.zodiac_start - self.cycle_offset
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

    fn planet_markers(&self) -> Html {
        let optimized_position = optimize::optimize(self.positions.planets());
        html! {
            <g>
                { for self.positions.planets().iter().enumerate().map(|(i, a)| {
                    let delta = optimized_position[i] - a;
                    let text_trans = format!(
                        "rotate({}) translate(0.775, 0) rotate({}) scale(0.008)",
                        -delta,
                        a + delta - self.zodiac_start - self.cycle_offset);
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

fn chord_path(radius: f64, start: f64, end: f64) -> String {
    let x0 = radius * start.to_radians().cos();
    let y0 = -radius * start.to_radians().sin();
    let x1 = radius * end.to_radians().cos();
    let y1 = -radius * end.to_radians().sin();
    format!("M {} {} L {} {}", x0, y0, x1, y1)
}
