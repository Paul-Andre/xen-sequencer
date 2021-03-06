// Object representing the pitch as in height of note

use std::rc::Rc;
use super::tuning::Tuning;
use super::interval::Interval;

pub struct ScalePitch {
    pub tuning: Rc<Tuning>,
    pub range: i32, //number of repeating intervals from reference frequency
    pub scale_degree: i32, // the number of the note in the scale
    pub accidentals_count: Vec<i32>,
    pub adjustment: Interval,
}

impl ScalePitch {
    pub fn get_frequency(&self) -> f64 {
        ( (self.range as f64) 
            * self.tuning
                  .get_repeating_interval()
                  .in_octaves() //is f64
	    + if self.scale_degree == 0 {
            0 as f64
        } else {
            self.tuning
            .scale[(self.scale_degree - 1) as usize]
            .interval
            .in_octaves() //is f64
        }
	    + self.accidentals_count
                  .iter()
                  .enumerate()
	          .map(|(i,n)| self.tuning
                                   .accidentals[i]
                                   .interval
                                   .in_octaves() //is f64
                                   * ((*n) as f64) )
	          .fold(0.0, |sum: f64, x: f64| sum + x) 
	    + self.adjustment
                  .in_octaves() ) //is f64
	.exp2() 
        * self.tuning
              .reference_frequency //is f64
    }
}



