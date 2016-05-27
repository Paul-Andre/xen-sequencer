// Object representing an interval

#[derive(Copy, Clone)]
pub struct Interval {
    pitch: f64, // for now, calculated in octaves
}

impl Interval {
    pub fn in_octaves(&self) -> f64 {
        self.pitch
    }
}
