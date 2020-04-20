use regex::Regex;
use crate::app::Positions;

const CONTENTS: &str = r#"
Sun     	22°14'35.78"Cap	  1° 1' 9"	IX
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
    static ref ZET9: Regex = Regex::new(r#"(\w+)\s+(\d+)°(\d+)'(\d+\.\d+)"(\w+)"#).unwrap();
}

fn to_num(ml: &MatchLine) -> f64 {
    let deg: f64 = ml.degrees.parse().unwrap();
    let min: f64 = ml.minutes.parse().unwrap();
    let sec: f64 = ml.seconds.parse().unwrap();
    deg + (min / 60.) + (sec / 3600.)
}

fn parse_zet9(text: &str) -> Result<Positions, &'static str> {
    let mut positions = Positions::default();
    for caps in ZET9.captures_iter(text) {
        let ml = MatchLine {
            name: caps.get(1).unwrap().as_str(),
            degrees: caps.get(2).unwrap().as_str(),
            minutes: caps.get(3).unwrap().as_str(),
            seconds: caps.get(4).unwrap().as_str(),
            sign: caps.get(5).unwrap().as_str(),
        };
        let position = to_num(&ml);
        match ml.name {
            "Sun" => positions.0[0] = position,
            "Moon" => positions.0[1] = position,
            "Mercury" => positions.0[2] = position,
            "Venus" => positions.0[3] = position,
            "Mars" => positions.0[4] = position,
            "Jupiter" => positions.0[5] = position,
            "Saturn" => positions.0[6] = position,
            "Uranus" => positions.0[7] = position,
            "Neptune" => positions.0[8] = position,
            "Pluto" => positions.0[9] = position,
            "Node" => positions.0[10] = position,
            "X" => positions.0[11] = position,
            "I" => positions.0[12] = position,
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
