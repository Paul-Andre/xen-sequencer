//mod synth;
use synth::{Synth, SynthFactory};

use std;
use std::sync::Arc;

struct BasicSynthFactory {
    wavetable: Arc<Vec<f32>>,
    frame_rate: u32
}

struct BasicSynth {
    wavetable: Arc<Vec<f32>>,
    frame_rate: u32,
    voice: Voice
}

#[derive(Default)]
struct Oscillator {
    phase: u32,
    delta: u32,
    amplitude: f64
}

#[derive(Default)]
struct Voice {
    note_id: u32,
    amplitude: f64,
    frequency: f64,
    on: bool,
    oscillators: [Oscillator;10]
}
    

impl SynthFactory for BasicSynthFactory {
    fn make_synth(&self) -> Box<Synth> {
        Box::new(BasicSynth{
            wavetable: self.wavetable.clone(),
            frame_rate: self.frame_rate,
            voice: Default::default()
        })
    }
}


fn frequency_to_u32_delta(frequency: f64, frame_rate: f64) -> u32 {
    // TODO cache an inverse frame rate when the synth is loaded to make this faster
    ((frequency / frame_rate) * (std::u32::MAX as u64 + 1) as f64 ) as u32
}


impl Synth for BasicSynth {
    fn get_audio_frame(&mut self) -> (f32, f32) {
        let voice = &mut (self.voice);
        let mut accumulator: f64 = 0.;

        if voice.on {
            for osc in voice.oscillators.iter_mut() {
                // In this part, I use the phase property to lookup a value in the wavetable.
                // I use the top 10 bits to look in the table, and I use the rest of the bits to
                // interpolate to neighboring values in the wavetable.
                // This technique works especially well for sine waves and when the table is big
                // enough, you can't really hear any difference.
                // TODO let the size of the wavetable be any power of 2, not just 1024

                let lookup_position_1: usize = (osc.phase >> (32 - 10)) as usize;
                let lookup_position_2: usize = (lookup_position_1 + 1) % 1024;

                let interpolation_mask: u32 = ( 1 << (32 - 10 ) ) - 1;
                let interpolation_denominator = 1 << (32 - 10 );
                let interpolation: f64 = (osc.phase & interpolation_mask) as f64 *
                    (1. / interpolation_denominator as f64);

                let value_1 = self.wavetable[lookup_position_1] as f64;
                let value_2 = self.wavetable[lookup_position_2] as f64;


                let interpolated_value = value_1 * (1.-interpolation) + value_2 * interpolation;
                accumulator += osc.amplitude * interpolated_value;

                osc.phase = osc.phase.wrapping_add(osc.delta);
            }
            accumulator *= voice.amplitude;
        }

        (accumulator as f32, accumulator as f32)
    }

    fn get_number_of_note_params(&self) -> u32 {
        2
    }

    fn get_note_param_name(&self, id: u32) -> Option<String> {
        match id {
            0 => Some("Amplitude".to_string()),
            1 => Some("Frequency".to_string()),
            _ => None
        }
    }

    fn note_on(&mut self, note_id: u32, delay: u32, note_params: Vec<Option<f64>>) {
        let voice = &mut (self.voice);
        voice.note_id = note_id;
        voice.amplitude = note_params[0].unwrap_or(0.5);
        voice.frequency = note_params[1].unwrap_or(100.);
        voice.on = true;

        for (i, osc) in voice.oscillators.iter_mut().enumerate() {
            osc.delta = frequency_to_u32_delta(voice.frequency*(i+1)as f64, self.frame_rate as f64);
            osc.phase = osc.delta.wrapping_mul(delay);
            osc.amplitude = 1./(i+1)as f64;
        }
    }

    fn note_off(&mut self, note_id: u32) {
        self.voice.on = false;
    }
}

pub fn make_basic_synth_factory(frame_rate: u32) -> Box<SynthFactory> {
    use std;
    let pi = std::f64::consts::PI;
    Box::new(BasicSynthFactory{
        wavetable: Arc::new (
                (0..1024).map(|i| (i as f64 /1024. * pi * 2. ).sin() as f32).collect() 
                ),
        frame_rate:frame_rate
    })
}

