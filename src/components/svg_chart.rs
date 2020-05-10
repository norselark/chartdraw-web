use itertools::Itertools;
use yew::prelude::*;

use super::drawing::HarmonicCycle;
use crate::app::Positions;
use crate::aspect;
use crate::optimize;

const ZODIAC_GLYPHS: [char; 12] = [
    '♈', '♉', '♊', '♋', '♌', '♍', '♎', '♏', '♐', '♑', '♒', '♓',
];
const PLANET_GLYPHS: [char; 11] = ['☉', '☽', '☿', '♀', '♂', '♃', '♄', '♅', '♆', '♇', '☊'];

// Radii
struct Radii {
    outer_zodiac: f32,
    inner_zodiac: f32,
    outer_houses: f32,
    inner_houses: f32,
    aspects: f32,
}

impl Default for Radii {
    fn default() -> Self {
        let outer_zodiac = 104.3;
        let inner_zodiac = 93.;
        let outer_houses = 86.3;
        let inner_houses = outer_houses - 16.;
        let aspects = 65.6;
        Self {
            outer_zodiac,
            inner_zodiac,
            outer_houses,
            inner_houses,
            aspects,
        }
    }
}

pub struct SvgChart {
    // link: ComponentLink<Self>,
    harmonic_cycle: HarmonicCycle,
    positions: Positions,
    zodiac_start: f32,
    cycle_offset: f32,
    aspect: bool,
    radii: Radii,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub harmonic_cycle: HarmonicCycle,
    pub positions: Positions,
    pub aspect: bool,
}

impl Component for SvgChart {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let zodiac_start = (props.positions.ascendant() - 180.) % 360.;
        let cycle_offset = 30.
            * match props.harmonic_cycle {
                HarmonicCycle::Cycle(n) => f32::from(n) - 1.,
                _ => 0.,
            };
        Self {
            // link,
            harmonic_cycle: props.harmonic_cycle,
            positions: props.positions,
            zodiac_start,
            cycle_offset,
            aspect: props.aspect,
            radii: Radii::default(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.cycle_offset = match props.harmonic_cycle {
            HarmonicCycle::Cycle(n) => 30. * f32::from(n),
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
            <svg class="img-fluid"
                version="1.1" baseProfile="full"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="-105 -105 210 210"
            >
                <circle r=self.radii.outer_houses stroke="white" fill="#55ffff" />
                <circle r=self.radii.inner_houses stroke="white" fill="#5555ff" />
                { house_sectors() }
                // Blue semicircle under horizon
                <g transform=format!("rotate({})", self.cycle_offset)>
                    <path d="M -70.3 0 A 70.3 70.3 0 0 0 70.3 0"
                        fill="#0000aa" stroke="white" />
                </g>
                <g transform=format!("rotate({})", self.zodiac_start + self.cycle_offset)>
                    { five_deg_lines() }
                    { self.zodiac_sectors() }
                    { self.planet_markers() }
                </g>
                <circle r=self.radii.outer_houses + 2. stroke="white" fill="transparent" />
                <g stroke="#00aaaa" fill="transparent">
                    <circle r=self.radii.outer_zodiac />
                    <circle r=self.radii.inner_zodiac />
                </g>
                // Ascendant arrow
                <g transform=format!("rotate({})", self.cycle_offset) stroke="white">
                    <path d="M 70.3 0 H 21.9 M -21.9 0 H -70.3 M -86.3 0 h -4"
                        stroke="white" />
                    <path d="M -90 1 h -2 l -3 -1 l 3 -1 h 2 z "
                        fill="white" />
                </g>
                // Descendant arrow
                <g transform=format!(
                    "rotate({})",
                    self.zodiac_start - self.positions.descendant() + self.cycle_offset
                ) stroke="white">
                    <path d="M 70.3 0 H 21.9 M -21.9 0 H -70.3 M 86.3 0 h 4"
                        stroke="white" />
                    <path d="M 90 1 h 2 l 3 -1 l -3 -1 h -2 z "
                        stroke="white" fill="black" />
                </g>
                // Centre disk
                {
                    if let HarmonicCycle::Cycle(_) = self.harmonic_cycle {
                        self.mini_horizon()
                    } else {
                        html! { <>
                            <circle r=21.9 stroke="black" fill="#55ffff" />
                            <circle r=0.6 fill="black" />
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

impl SvgChart {
    fn mini_horizon(&self) -> Html {
        let sun_transform = format!("rotate({})", self.zodiac_start - self.positions.sun());
        let moon_transform = format!("rotate({})", self.zodiac_start - self.positions.moon());
        html! {
            <>
                <circle r=21.9 stroke="white" fill="#5555ff" />
                <path d="M -21.9 0 A 21.9 21.9 0 0 0 21.9 0" fill="#0000aa" stroke="white" />
                <path d="M -21.9 0 H 21.9 " stroke="white" />
                <path d="M -21.9 1 h -2 l -3 -1 l 3 -1 h 2 z " stroke="white" fill="white" />
                <g transform=format!("rotate({})", self.zodiac_start - self.positions.descendant()) stroke="white">
                    <path d="M -21.9 0 H 21.9 " />
                    <path d="M 21.9 1 h 2 l 3 -1 l -3 -1 h -2 z " fill="black" />
                </g>
                <circle cx=16 r=1.4 stroke="black" fill="yellow" transform=sun_transform />
                <circle cx=16 r=1.4 stroke="black" fill="lightgrey" transform=moon_transform />
                <circle r=3 stroke="black" fill="#55ffff" />
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
                    if matches!(asp.aspect_type, aspect::Type::Zero | aspect::Type::Thirty) =>
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
            <circle r=self.radii.aspects fill="white" />
            <g transform=format!("rotate({})", self.zodiac_start + self.cycle_offset)>
                { for aspect_pairs.map(|(a, b, aspect)| {
                    let stroke = match aspect.aspect_type {
                        aspect::Type::Ninety | aspect::Type::OneEighty => "#aa0000",
                        _ => "#00aa00",
                    };
                    let width = 0.5 + 1.2 * aspect.close;
                    html! {
                        <path d=chord_path(self.radii.aspects, a, b) stroke=stroke stroke-width=width />
                    }
                } ) }
            </g>
            <circle r=4 fill="#5555ff" stroke="black" />
            <path d="M -4 0 A 4 4 0 0 0 4 0" fill="#0000aa" stroke="black" transform=cycle_rot />
            <path d="M -4 0 H 4" stroke="white" transform=desc_rot />
            <path d="M -4 0 H 4" stroke="white" transform=asc_rot />
            <circle r="1.2" fill="#black" stroke="#55ffff" />
            </>
        }
    }

    /// `start_of_zodiac` is passed in order to flip the glyphs right side up
    fn zodiac_sectors(&self) -> Html {
        let sector = |offset| {
            let angle = (30 * offset) as f32;
            let rotation = format!("rotate({})", -angle);
            let text_trans = format!(
                "rotate(-15) translate(98, 0) rotate({}) scale(0.8)",
                angle + 15. - self.zodiac_start - self.cycle_offset
            );
            let glyph = ZODIAC_GLYPHS[offset];
            html! {
                <g transform=rotation>
                    <path d="M 93 0 H 104.3" stroke="#00aaaa" />
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
                        "rotate({}) translate(77.5, 0) rotate({}) scale(0.8)",
                        -delta,
                        a + delta - self.zodiac_start - self.cycle_offset);
                    let glyph = PLANET_GLYPHS[i];
                    html! {
                        <g transform=format!("rotate({})", -a) stroke-width="0.5">
                            <circle cx="68.4" r="1.2" stroke="white" fill="transparent" />
                            <circle cx="90.2" r="1.5" fill="white" />
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
                    html!{<path d="M 70.3 0 H 86.3" transform=rotation />}
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
                    <path d="M 86.3 0 H 88.3" transform=rotation />
                }
            }) }
        </g>
    }
}

fn chord_path(radius: f32, start: f32, end: f32) -> String {
    let x0 = radius * start.to_radians().cos();
    let y0 = -radius * start.to_radians().sin();
    let x1 = radius * end.to_radians().cos();
    let y1 = -radius * end.to_radians().sin();
    format!("M {} {} L {} {}", x0, y0, x1, y1)
}
