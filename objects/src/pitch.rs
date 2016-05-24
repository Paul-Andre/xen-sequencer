// Object representing the pitch as in height of note

pub struct Pitch {
    tuning: Rc<Tuning>,
    range: i32, //number of repeating intervals from reference frequency
    scale_degree: i32, // the number of the note in the scale
    accidentals_count: Vec<i32>,
    adjustment: Interval,
}
