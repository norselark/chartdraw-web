use crate::app::Positions;
use regex::Regex;

pub const CONTENTS: &str = r#"Sun     	22°14'35.78"Cap	  1° 1' 9"	IX
Moon    	 2°39'38.96"Sgr	 11°57'25"	VI
Mercury 	 1°45'01.32"Cap	  1°21'23"	VII
Venus   	23°00'48.61"Cap	  1°15'28"	IX
Mars    	21°20'04.68"Sco	  0°37'15"	VI
Jupiter 	18°45'55.73"Sco	  0° 8'54"	VI
Saturn  	 2°43'17.49"Cap	  0° 6'49"	VII
Uranus  	24°36'45.68"Ari	  0° 0'31"	XII
Neptune 	12°11'05.10"Psc	  0° 1'37"	XI
Pluto   	19°10'15.12"Cap	  0° 2' 3"	VIII
Node    	15°09'50.25"Leo	- 0° 3'42"	IV
Lilith  	 7°10'07.07"Cap	  0° 6'41"	VII

I	27°22'14.79"Gem
II	 9°32'04.66"Cnc
III	21°31'12.67"Cnc
IV	 6°23'53.77"Leo
V	29°48'25.89"Leo
VI	18°51'29.91"Lib
VII	27°22'14.79"Sgr
VIII	 9°32'04.66"Cap
IX	21°31'12.67"Cap
X	 6°23'53.77"Aqr
XI	29°48'25.89"Aqr
XII	18°51'29.91"Ari"#;

#[derive(Debug)]
struct MatchLine<'a> {
    name: &'a str,
    degrees: &'a str,
    minutes: &'a str,
    seconds: &'a str,
    sign: &'a str,
}

lazy_static! {
    /// A pattern to match lines from ZET9's export format
    static ref ZET9_PAT: Regex = Regex::new(r#"(\w+)\s+(\d+)°(\d+)'(\d+\.\d+)"(\w+)"#).unwrap();
}

/// The abbreviated zodiac names used by ZET9
const ZET9_ZODIAC: [&str; 12] = [
    "Ari", "Tau", "Gem", "Cnc", "Leo", "Vir", "Lib", "Sco", "Sgr", "Cap", "Aqr", "Psc",
];

#[derive(Debug)]
pub enum Error {
    UnknownZodiacSign(String),
}

fn to_num(ml: &MatchLine) -> Result<f64, Error> {
    let zodiac_idx = ZET9_ZODIAC
        .iter()
        .position(|&e| e == ml.sign)
        .ok_or(Error::UnknownZodiacSign(ml.sign.to_string()))? as f64;
    // These are matched as digit sequences and should always parse successfully
    let deg: f64 = ml.degrees.parse().unwrap();
    let min: f64 = ml.minutes.parse().unwrap();
    let sec: f64 = ml.seconds.parse().unwrap();
    Ok(30. * zodiac_idx + deg + (min / 60.) + (sec / 3600.))
}

pub fn parse_zet9(text: &str) -> Result<Positions, Error> {
    let mut positions = Positions::default();
    for caps in ZET9_PAT.captures_iter(text) {
        // All groups must be present for caps to exist
        let ml = MatchLine {
            name: caps.get(1).unwrap().as_str(),
            degrees: caps.get(2).unwrap().as_str(),
            minutes: caps.get(3).unwrap().as_str(),
            seconds: caps.get(4).unwrap().as_str(),
            sign: caps.get(5).unwrap().as_str(),
        };
        let mut set_position = |i: usize| {
            positions.0[i] = to_num(&ml)?;
            Ok(())
        };
        match ml.name {
            "Sun" => set_position(0)?,
            "Moon" => set_position(1)?,
            "Mercury" => set_position(2)?,
            "Venus" => set_position(3)?,
            "Mars" => set_position(4)?,
            "Jupiter" => set_position(5)?,
            "Saturn" => set_position(6)?,
            "Uranus" => set_position(7)?,
            "Neptune" => set_position(8)?,
            "Pluto" => set_position(9)?,
            "Node" => set_position(10)?,
            "X" => set_position(11)?,
            "I" => set_position(12)?,
            _ => (),
        }
    }
    Ok(positions)
}

pub fn main() {
    let positions = parse_zet9(CONTENTS);
    if let Ok(p) = positions {
        web_sys::console::log_1(&format!("{:?}", p).into());
    }
}
