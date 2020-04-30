use crate::app::Positions;
use yew::prelude::*;

lazy_static! {
    static ref NAMES: Vec<&'static str> = vec![
        "\u{2609}", "\u{263d}", "\u{263f}", "\u{2640}", "\u{2642}", "\u{2643}", "\u{2644}",
        "\u{2645}", "\u{2646}", "\u{2647}", "\u{260a}", "MC", "ASC",
    ];
}

const NUM: usize = 13;

pub struct ListView {
    link: ComponentLink<Self>,
    positions: Positions,
    angle_format: AngleFormat,
    offset: usize,
}

enum AngleFormat {
    Truncated,
    Full,
}

lazy_static! {
    static ref ZODIAC_SIGNS: Vec<&'static str> =
        vec!["Ari", "Tau", "Gem", "Can", "Leo", "Vir", "Lib", "Sco", "Sag", "Cap", "Aqu", "Psc",];
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub positions: Positions,
}

pub enum Msg {
    Toggle,
    IncOffset,
    DecOffset,
}

impl Component for ListView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            positions: props.positions,
            angle_format: AngleFormat::Truncated,
            offset: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => match self.angle_format {
                AngleFormat::Full => self.angle_format = AngleFormat::Truncated,
                AngleFormat::Truncated => self.angle_format = AngleFormat::Full,
            },
            Msg::IncOffset => self.offset = (self.offset + NUM - 1) % NUM,
            Msg::DecOffset => self.offset = (self.offset + 1) % NUM,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.positions = props.positions;
        true
    }

    fn view(&self) -> Html {
        let on_toggle = self.link.callback(|_| Msg::Toggle);
        let on_scrollup = self.link.callback(|_| Msg::DecOffset);
        let on_scrolldown = self.link.callback(|_| Msg::IncOffset);

        let indices = (self.offset..NUM).chain(0..self.offset);
        let pos_vec = self.positions.0.to_vec();
        html! {
            <div>
                <table class="table table-sm table-hover">
                    { for indices.map(|i| self.format_row(NAMES[i], pos_vec[i])) }
                </table>
                <div class="btn-group" role="group">
                    <button class="btn" onclick=on_scrollup>{ '▲' }</button>
                    <button class="btn" onclick=on_toggle >{ "Toggle" }</button>
                    <button class="btn" onclick=on_scrolldown>{ '▼' }</button>
                </div>
            </div>
        }
    }
}

impl ListView {
    fn format_row(&self, name: &str, angle: f32) -> Html {
        let text = match self.angle_format {
            AngleFormat::Truncated => truncate_angle(angle),
            AngleFormat::Full => full_angle(angle),
        };
        html! {
            <tr>
                <td>{ name }</td>
                <td>{ text }</td>
            </tr>
        }
    }
}

fn truncate_angle(angle: f32) -> String {
    assert!(angle >= 0.);
    let mut int_part = angle.floor();
    let decimal_part: f32 = angle - int_part;
    let zodiac_sign = ZODIAC_SIGNS[int_part as usize / 30];
    let mut i = (decimal_part * 60.).round();
    int_part %= 30.;
    if i > 59. {
        i -= 60.;
        int_part += 1.;
    }
    format!("{:02}° {:02}' {}", int_part, i, zodiac_sign)
}

fn full_angle(angle: f32) -> String {
    format!("{:.2}°", angle)
}
