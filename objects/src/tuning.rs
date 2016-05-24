// Object representing tuning

use super::interval::Interval;

struct ScaleNote {
    interval: Interval,
    name: String,
}

struct Accidental {
    interval: Interval,
    name: String,
}
    

struct Tuning {
    scale: Vec<ScaleNote>,
    accidentals: Vec<Accidental>,
    reference_frequency: f64,
    name: String,
}

impl Tuning {
    fn get_repeating_interval(&self) -> Interval {
        self.scale[self.scale.length()].interval
    }
}
