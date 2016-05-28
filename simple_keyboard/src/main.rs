extern crate sdl2;
extern crate synth_interface;
extern crate basic_synth;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};

use synth_interface::{Synth, SynthFactory};


mod keyboard_map;
mod palette;


use std::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;


// TODO make some kind of generalized event objects or something.
enum KeyboardEvent {
    On {note_id: u32, frequency: f64},
    Off {note_id: u32}
}

struct SynthPlayer {
    synth: Box<Synth>,
    note_receiver: mpsc::Receiver<KeyboardEvent>,
    frames_passed: Arc<AtomicUsize>,
}


impl AudioCallback for SynthPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut frames_passed = self.frames_passed.load(std::sync::atomic::Ordering::Relaxed);

        let mut previous_frame: (f32, f32) = (0., 0.);
        while let Ok(KeyboardEvent) = self.note_receiver.try_recv() {
            match KeyboardEvent {
                KeyboardEvent::On {note_id: note_id, frequency: frequency} => {
                    self.synth.note_on(note_id, 0, &[Some(0.5), Some(frequency)]);
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
                frames_passed += 1;
            }
            else {
                *x=previous_frame.1;
            }
        }

        self.frames_passed.store(frames_passed, std::sync::atomic::Ordering::Relaxed);
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    //let timer_subsystem = sdl_context.timer().unwrap();

    let mut track: Vec<Option<f64>> = vec![None;16];

    track[0] = Some(440.);
    track[5] = Some(440.);


    let (note_sender, note_receiver) = mpsc::channel();
    //let (synch_sender, synch_receiver) = mpsc::channel();

    let frames_passed = Arc::new(AtomicUsize::new(0));

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),
        samples: Some(256)
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);

        let synth_factory = basic_synth::make_basic_synth_factory(spec.freq as f64);

        SynthPlayer {
            synth: synth_factory.make_synth(),
            note_receiver: note_receiver,
            frames_passed: frames_passed.clone(),
        }
        
    }).unwrap();

    device.resume();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 700, 480)
        .position_centered()
        //.opengl()
        .build()
        .unwrap();




    let mut renderer = window.renderer().build().unwrap();

    let square_size = 30;
    let gap = 10;
    let pod_gap = 0;
    let padding = 10;

    renderer.set_draw_color(palette::background1());
    renderer.clear();

    renderer.set_draw_color(palette::background2());
/*
    for i in 0..4 {
        renderer.fill_rect( sdl2::rect::Rect::new(
                i*(4*(square_size+gap)+pod_gap) - padding,
                200-padding,
                (4*square_size+3*gap+2*padding) as u32,
                (square_size+2*padding) as u32
        )).unwrap();
    }

    */
    renderer.set_draw_color(palette::inactive());

    for i in 0..16 {
        renderer.fill_rect( sdl2::rect::Rect::new(
                35 + i*(square_size+gap) + (i/4)*pod_gap,
                200,
                square_size as u32,
                square_size as u32
        )).unwrap();

        renderer.fill_rect( sdl2::rect::Rect::new(
                35 + i*(square_size+gap) + square_size/2 + (i/4)*pod_gap - 5,
                200 - 4 - 4,
                10 as u32,
                4 as u32
        )).unwrap();
    }

    renderer.present();


    let mut event_pump = sdl_context.event_pump().unwrap();

    fn calculate_frequency(coordinates: (i32, i32)) -> f64 {
        let left_pitch = 5./31.;
        let up_pitch = 3./31.;
        ((coordinates.0 - 4) as f64 * left_pitch
         + (coordinates.1 - 1) as f64 * up_pitch).exp2() * 440.
    }


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { scancode: Some(scancode), repeat: false,  .. } => {
                    if let Some(coordinates) = keyboard_map::map_scancode(scancode) {
                        note_sender.send(KeyboardEvent::On{
                            note_id: (coordinates.1 * 1024 + coordinates.0) as u32,
                            frequency: calculate_frequency(coordinates)
                        });
                        renderer.set_draw_color(palette::active());

                        //renderer.fill_rect(sdl2::rect::Rect::new(100, 100, 50, 50)).unwrap();
                        renderer.fill_rect(sdl2::rect::Rect::new(640 - 150, 200, 50, 50)).unwrap();
                    }
                }
                Event::KeyUp { scancode: Some(scancode), .. } => {
                    if let Some(coordinates) = keyboard_map::map_scancode(scancode) {
                        note_sender.send(KeyboardEvent::Off{
                            note_id: (coordinates.1 * 1024 + coordinates.0) as u32
                        });
                        renderer.set_draw_color(palette::inactive());

                        //renderer.fill_rect(sdl2::rect::Rect::new(100, 100, 50, 50)).unwrap();
                        renderer.fill_rect(sdl2::rect::Rect::new(640 - 150, 200, 50, 50)).unwrap();
                    }
                }
                Event::Window { win_event_id: sdl2::event::WindowEventId::Exposed, .. } => {
                    renderer.present();
                }

                _ => {
                }
            }
        }
        
        renderer.present();
        //println!("{}",frames_passed.load(std::sync::atomic::Ordering::Relaxed));
    }
}
