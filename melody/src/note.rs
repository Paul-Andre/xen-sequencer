/// Defines a structure for Note.

use scale_pitch::ScalePitch;

pub struct Note {

    // start and duration will represents seconds for now but in the future it would be best to
    // make them be represented by a special object.
    pub start: f64,
    pub duration: f64,

    pub pitch: ScalePitch,
    pub amplitude: f64,
}

