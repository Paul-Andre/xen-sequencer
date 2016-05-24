// Object representing tuning

struct ScaleNote {
    interval: Interval,
    name: String,
}

struct Accidental {
    interval: Interval,
    name: String,
}
    

struct Tuning {
    scale: Vec<ScaleNote>
    accidentals: Vec<Accidental>
    reference_frequency: f64,
    name: String,
}


