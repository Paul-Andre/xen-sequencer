extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

mod synth;
use synth::{Synth, SynthFactory};

mod basic_synth;

mod keyboard_map;


use std::sync::mpsc;

// TODO make some kind of generalized event objects or something.
enum KeyboardEvent {
    On {note_id: u32, frequency: f64},
    Off {note_id: u32}
}

struct SynthPlayer {
    synth: Box<Synth>,
    communication_channel: mpsc::Receiver<KeyboardEvent>
}


// These notes are just for testing.
static notes: [f64; 8] = [100., 200., 300., 400.,  500., 100., 350., 400. ];

impl AudioCallback for SynthPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {

        let mut previous_frame: (f32, f32) = (0., 0.);
        while let Ok(KeyboardEvent) = self.communication_channel.try_recv() {
            match KeyboardEvent {
                KeyboardEvent::On {note_id: note_id, frequency: frequency} => {
                    self.synth.note_on(note_id, 0, vec![Some(0.5), Some(frequency)]);
                }
                KeyboardEvent::Off {note_id: note_id} => {
                    self.synth.note_off(note_id);
                }
            }
        }
        for (i, x) in out.iter_mut().enumerate() {
            if i%2==0 {
                previous_frame = self.synth.get_audio_frame();
                *x=previous_frame.0;
            }
            else {
                *x=previous_frame.1;
            }
        }
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let (tx, rx) = mpsc::channel();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),
        samples: None
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);

        let synth_factory = basic_synth::make_basic_synth_factory(spec.freq as u32);

        SynthPlayer {
            synth: synth_factory.make_synth(),
            communication_channel: rx
        }
        
    }).unwrap();

    device.resume();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 80, 60)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(255, 0, 0));
    renderer.clear();
    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    fn calculate_frequency(coordinates: (i32, i32)) -> f64 {
        let left_pitch = 2./12.;
        let up_pitch = 1./12.;
        ((coordinates.0 - 4) as f64 * left_pitch
         + (coordinates.1 - 1) as f64 * up_pitch).exp2() * 440.
    }

    'running: loop {
        for event in event_pump.wait_timeout_iter(10) {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(scancode), repeat: false,  .. } => {
                    if let Some(coordinates) = keyboard_map::map_scancode(scancode) {
                        tx.send(KeyboardEvent::On{
                            note_id: (coordinates.1 * 1024 + coordinates.0) as u32,
                            frequency: calculate_frequency(coordinates)
                        });
                    }
                }
                Event::KeyUp { scancode: Some(scancode), .. } => {
                    if let Some(coordinates) = keyboard_map::map_scancode(scancode) {
                        tx.send(KeyboardEvent::Off{
                            note_id: (coordinates.1 * 1024 + coordinates.0) as u32
                        });
                    }
                }
                _ => {}
            }
        }
    }
}
