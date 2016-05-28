// Object representing tuning

use super::interval::Interval;

pub struct ScaleNote {
    pub interval: Interval,
    pub name: String,
}

pub struct Accidental {
    pub interval: Interval,
    pub name: String,
}
    

pub struct Tuning {
    pub scale: Vec<ScaleNote>,
    pub accidentals: Vec<Accidental>,
    pub reference_frequency: f64,
    pub name: String,
}

impl Tuning {
    pub fn get_repeating_interval(&self) -> Interval {
        self.scale[self.scale.len()-1]
            .interval
            .clone()
    }
}
