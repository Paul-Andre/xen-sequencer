//mod synth;
use synth::{Synth, SynthFactory};

use std::sync::Arc;

struct BasicSynthFactory{
    sine_wavetable: Arc<Vec<f32>>
}

struct BasicSynth{
    sine_wavetable: Arc<Vec<f32>>
}
    

impl SynthFactory for BasicSynthFactory {
    fn make_synth(&self) -> Box<Synth> {
        Box::new(BasicSynth{sine_wavetable: self.sine_wavetable.clone()})
    }
}

impl Synth for BasicSynth {
    fn get_audio_frame(&mut self) -> (f32, f32) {
        (0., 0.)
    }
}

pub fn make_BasicSynthFactory() -> Box<SynthFactory> {
    use std;
    let pi = std::f64::consts::PI;  // why doesn't this work??
    //let pi = 3.14159265358979323846264338327950288f64;
    Box::new(BasicSynthFactory{
        sine_wavetable: Arc::new (
                (0..1024).map(|i| (i as f64 /1024. * pi * 2. ).sin() as f32).collect() 
                )
    })
}

