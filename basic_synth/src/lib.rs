extern crate synth_interface;
use synth_interface::{Synth, SynthFactory};

use std::sync::Arc;

struct BasicSynthFactory {
    wavetable: Arc<Vec<f64>>,
    frame_rate: f64,
}

struct BasicSynth {
    wavetable: Arc<Vec<f64>>,
    frame_rate: f64,
    voices: [Voice;16],
    last_used_voice: usize,
    delay_line: Vec<f64>,
    delay_line_pointer: usize,
}

#[derive(Default,Debug)]
struct Oscillator {
    phase: u32,
    delta: u32,
    amplitude: f64,
    frequency_multiplier: f64,
    default_amplitude: f64,
    decay_factor: f64,
}

#[derive(PartialEq,Eq,Debug)]
enum State {
    On,
    Released,
    Off
}

use self::State::*;

impl Default for State {
    fn default() -> State { State::Off }
}

#[derive(Debug)]
struct Voice {
    note_id: u32,
    amplitude: f64,
    frequency: f64,
    state: State,
    oscillators: [Oscillator;10]
}

impl Default for Voice {
    fn default() -> Voice{
        let mut ret = Voice {
            note_id: 12341234,
            amplitude: 0.,
            frequency: 0.,
            state: State::default(),
            oscillators: Default::default()
        };

        let mut decay_factor: f64  = 1.;
        for (i, osc) in ret.oscillators.iter_mut().enumerate() {
            osc.frequency_multiplier = (i+1) as f64;
            osc.default_amplitude = 0.2/(i+1) as f64;
            osc.decay_factor = decay_factor;
            decay_factor *= 0.99998;
        }
        return ret;
    }
}

    

impl SynthFactory for BasicSynthFactory {
    fn make_synth(&self) -> Box<Synth> {
        Box::new(BasicSynth{
            wavetable: self.wavetable.clone(),
            frame_rate: self.frame_rate,
            voices: Default::default(),
            last_used_voice: 0,
            delay_line: vec![0 as f64 ; 100_000],
            delay_line_pointer: 0,
        })
    }
}


fn frequency_to_u32_delta(frequency: f64, frame_rate: f64) -> u32 {
    // TODO cache an inverse frame rate when the synth is loaded to make this faster
    ((frequency / frame_rate) * (std::u32::MAX as u64 + 1) as f64 ) as u32
}

impl BasicSynth {
    fn find_free_voice(&self) -> Option<usize> {
        // Goes through all voices.
        // If finds one that is off, returns.
        // Else returns the first voice that it found that was released.
        let mut released_voice = None;
        for i in 0..self.voices.len() {
            let j = (i + 1 + self.last_used_voice) % self.voices.len() as usize;
            match self.voices[j].state {
                Off => {
                    return Some(j);
                },
                Released => {
                    if released_voice == None {
                        released_voice = Some(j);
                    }
                },
                _ => {}
            }
        }
        return released_voice;
    }

}



fn u32_lookup(wavetable: &[f64], phase: u32) -> f64 {
    // In this part, I use the phase property to lookup a value in the wavetable.
    // I use the top 10 bits to look in the table, and I use the rest of the bits to
    // interpolate to neighboring values in the wavetable.
    // This technique works especially well for sine waves and when the table is big
    // enough, you can't really hear any difference.
    // TODO let the size of the wavetable be any power of 2, not just 1024

    let interpolation_mask: u32 = ( 1 << (32 - 10 ) ) - 1;
    let inv_interpolation_denominator: f64 = 1./ (1 << (32 - 10 )) as f64;


    let lookup_position_1: usize = (phase >> (32 - 10)) as usize;
    let lookup_position_2: usize = (lookup_position_1 + 1) % 1024;

    let interpolation: f64 = (phase & interpolation_mask) as f64 *
        inv_interpolation_denominator;

    let value_1 = wavetable[lookup_position_1];
    let value_2 = wavetable[lookup_position_2];

    value_1 * (1.-interpolation) + value_2 * interpolation
}

fn f64_lookup(wavetable: &[f64], phase: f64) -> f64 {
    let normalized_phase = if phase>=0. { phase % 1.0 } else { phase % 1.0 + 1.0 };


    let lookup_position_1: usize = (normalized_phase * 1024 as f64) as usize;
    let lookup_position_2: usize = (lookup_position_1 + 1) % 1024;

    let interpolation: f64 = (normalized_phase * 1024 as f64 % 1. ) as f64;

    let value_1 = wavetable[lookup_position_1];
    let value_2 = wavetable[lookup_position_2];

    value_1 * (1.-interpolation) + value_2 * interpolation
}

impl Synth for BasicSynth {
    fn get_audio_frame(&mut self) -> (f32, f32) {
        let mut total_accumulator: f64 = 0.;


        for voice in self.voices.iter_mut() {
            let mut voice_accumulator: f64 = 0.;

            if voice.state == On || voice.state == Released {
                for osc in voice.oscillators.iter_mut() {


                    voice_accumulator += osc.amplitude*
                        f64_lookup(&self.wavetable, 
                                      (osc.phase as f64)*(1./std::u32::MAX as f64)*osc.amplitude
                                      + u32_lookup(&self.wavetable, osc.phase));

                    osc.phase = osc.phase.wrapping_add(osc.delta);
                    osc.amplitude *= osc.decay_factor;
                }
            }
            if voice.state == Released {
                voice.amplitude *= 0.9995;
                if voice.amplitude <= 0.0001 {
                    voice.state = Off;
                }
            }
            total_accumulator += voice_accumulator * voice.amplitude;
        }

        total_accumulator *= 0.6;
        /*
        total_accumulator += 0.5 * self.delay_line[
            ((self.delay_line_pointer + self.delay_line.len() - ( 10000.0)as usize)
             % self.delay_line.len())
            ];
            */
        self.delay_line[self.delay_line_pointer] = total_accumulator;
        self.delay_line_pointer = (self.delay_line_pointer + 1) % self.delay_line.len();
        (total_accumulator as f32, total_accumulator as f32)
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

    fn note_on(&mut self, note_id: u32, delay: u32, note_params: &[Option<f64>]) {
        let voice =
            if let Some(voice_id) = self.find_free_voice() {
                self.last_used_voice = voice_id;
                println!("found free: {}",voice_id);
                &mut (self.voices[voice_id])
            }
            else {
                self.last_used_voice = (self.last_used_voice + 1) % self.voices.len();
                println!("No found free: {}", self.last_used_voice);
                &mut (self.voices[self.last_used_voice])
            };

        voice.note_id = note_id;
        voice.amplitude = note_params[0].unwrap_or(0.5);
        voice.frequency = note_params[1].unwrap_or(100.);

        for (i, osc) in voice.oscillators.iter_mut().enumerate() {
            osc.delta = frequency_to_u32_delta(voice.frequency*osc.frequency_multiplier, self.frame_rate as f64);
            if voice.state == Off {
                osc.phase = osc.delta.wrapping_mul(delay);
            }
            osc.amplitude = osc.default_amplitude;
        }

        voice.state = On;
    }

    fn note_off(&mut self, note_id: u32) {
        for voice in self.voices.iter_mut() {
            if voice.note_id == note_id && voice.state == On {
                voice.state = Released;
                println!("released");
                //break;
            }
        }
    }
}

pub fn make_basic_synth_factory(frame_rate: f64) -> Box<SynthFactory> {
    use std;
    let pi = std::f64::consts::PI;
    Box::new(BasicSynthFactory{
        wavetable: Arc::new (
                (0..1024).map(|i| (i as f64 /1024. * pi * 2. ).sin()).collect() 
                ),
        frame_rate: frame_rate
    })
}

