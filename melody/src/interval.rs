// Object representing an interval

#[derive(Copy, Clone)]
pub struct Interval {
    pitch: f64, // for now, calculated in octaves
    representation: IntervalRepresentation,
}

impl Interval {
    pub fn in_octaves(&self) -> f64 {
        self.pitch
    }

    pub fn get_representation(&self) -> IntervalRepresentation {
        self.representation 
    }
}

enum IntervalRepresentation {
    Ratio(RatioPower<i32>),
    Cents(i32),
}

// RationalPower

use num::rational;

pub struct RatioPower<T> {
    exponent: Ratio<T>,
    base: Ratio<T>,
}

impl Mul<RatioPower<T>> for RatioPower<T> {
    type Output = RatioPower;

    pub fn mul(self, other: RatioPower<T>) -> RatioPower<T> {
        RatioPower<T> { exponent: self.exponent + other.exponent,
                        base: self.base * other.base, }
    }
}

