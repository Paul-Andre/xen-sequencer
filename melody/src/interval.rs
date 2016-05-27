// Object representing an interval

#[derive(Copy, Clone)]
pub struct Interval {
    pitch: f64, // for now, calculated in octaves
}

impl Interval {
    pub fn from_octaves(octaves: f64) -> Interval {
        Interval {
            pitch: octaves,
        }
    }
    pub fn in_octaves(&self) -> f64 {
        self.pitch
    }
}
