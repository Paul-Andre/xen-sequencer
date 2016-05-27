// Object representing an interval

extern crate num;

use std::ops::Mul;
use self::num::rational::Ratio;
use self::num::integer::Integer;

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

#[derive(Copy, Clone)]
pub enum IntervalRepresentation {
    Ratio(RatioPower<i32>),
    Cents(i32),
}

// RationalPower

#[derive(Copy, Clone)]
pub struct RatioPower<T> 
    where T: Clone + Integer 
{
    exponent: Ratio<T>,
    base: Ratio<T>,
}

impl<T> Mul<RatioPower<T>> for RatioPower<T>
    where T: Clone + Integer
{
    type Output = RatioPower<T>;

    fn mul(self, other: RatioPower<T>) -> RatioPower<T> {
        RatioPower::<T> { exponent: self.exponent + other.exponent,
                        base: self.base * other.base, }
    }
}

