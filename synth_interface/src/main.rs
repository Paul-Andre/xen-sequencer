extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
mod synth;
use synth::{Synth, SynthFactory};

mod BasicSynth;

struct SynthPlayer {
    synth: Box<Synth>
}

impl AudioCallback for SynthPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {

        let mut i: i32 = 0;
        let mut previousFrame: (f32, f32) = (0., 0.);
        for x in out.iter_mut() {
            if i%2==0 {
                previousFrame = self.synth.get_audio_frame();
                *x=previousFrame.0;
            }
            else {
                *x=previousFrame.1;
            }
            i+=1;
        }
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let synth_factory = BasicSynth::make_BasicSynthFactory();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),
        samples: None
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);

        SynthPlayer {
            synth: synth_factory.make_synth()
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
        for event in event_pump.poll_iter() {
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
