extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

mod synth;
use synth::{Synth, SynthFactory};

mod basic_synth;

struct SynthPlayer {
    synth: Box<Synth>,
    frames_passed_since_last_note: u32,
    current_note: usize
}


static notes: [f64; 8] = [100., 200., 300., 400.,  500., 234., 700., 450. ];

impl AudioCallback for SynthPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {


        let mut previous_frame: (f32, f32) = (0., 0.);
        for (i, x) in out.iter_mut().enumerate() {
            if i%2==0 {
                self.frames_passed_since_last_note += 1;
                if self.frames_passed_since_last_note >= 44100 / 8 {
                    self.frames_passed_since_last_note = 0;
                    self.current_note += 1;
                    if self.current_note >= notes.len() {
                        self.current_note = 0
                    }
                    self.synth.note_off(0);
                    self.synth.note_on(0,0, vec![Some(0.5), Some(notes[self.current_note])]);
                }
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
            frames_passed_since_last_note: 1000000,
            current_note: notes.len() - 1
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

    'running: loop {
        for event in event_pump.wait_timeout_iter(10) {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
    }

}
