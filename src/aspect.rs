pub struct Aspect {
    pub aspect_type: AspectType,
    pub close: f64,
}

pub enum AspectType {
    Zero,
    Thirty,
    Sixty,
    Ninety,
    OneTwenty,
    OneEighty,
}

impl Aspect {
    fn new(aspect_type: AspectType, close: f64) -> Aspect {
        Self { aspect_type, close }
    }
}

pub fn aspect(a: f64, b: f64, orbis: f64) -> Option<Aspect> {
    let distance = (a - b).abs();
    let distance = distance.min(360. - distance);
    if distance < orbis {
        let close = distance / orbis;
        Some(Aspect::new(AspectType::Zero, close))
    } else if (distance - 30.).abs() < 0.25 * orbis {
        let close = 1. - (30. - distance).abs() / (orbis * 0.25);
        Some(Aspect::new(AspectType::Thirty, close))
    } else if (distance - 60.).abs() < 0.75 * orbis {
        let close = 1. - (60. - distance).abs() / (orbis * 0.75);
        Some(Aspect::new(AspectType::Sixty, close))
    } else if (distance - 90.).abs() < orbis {
        let close = 1. - (90. - distance).abs() / orbis;
        Some(Aspect::new(AspectType::Ninety, close))
    } else if (distance - 120.).abs() < orbis {
        let close = 1. - (120. - distance).abs() / orbis;
        Some(Aspect::new(AspectType::OneTwenty, close))
    } else if (distance - 180.).abs() < orbis {
        let close = 1. - (180. - distance).abs() / orbis;
        Some(Aspect::new(AspectType::OneEighty, close))
    } else {
        None
    }
}
