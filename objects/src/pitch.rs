// Object representing the pitch as in height of note

use std::rc::Rc;
use super::tuning::Tuning;
use super::interval::Interval;

pub struct Pitch {
    tuning: Rc<Tuning>,
    range: i32, //number of repeating intervals from reference frequency
    scale_degree: i32, // the number of the note in the scale
    accidentals_count: Vec<i32>,
    adjustment: Interval,
}

impl Pitch {
    fn get_frequency(&self) -> f64 {
        (self.range*self.tuning.get_repeating_interval().get_interval_in_octaves()
	 + if self.scale_degree == 0 {0}
	 else {self.tuning.scale[self.scale_degree-1].interval.get_interval_in_octaves()}
	 + accidentals_count.to_iter().enumerate()
	 .map(|(i,n)| self.tuning.accidentals[i].interval.get_iterval_in_octaves()*n)
	 .fold(0, |sum, x| sum + x)
	 + self.adjustment.get_interval_in_octaves() )
	.exp2() * self.tuning.reference_frequency
    }
}



