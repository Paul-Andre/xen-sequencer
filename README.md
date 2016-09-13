# Sequencer

The idea was to make a fully microtonal/xenharmonic synthesizer and sequencer in Rust. We didn't come too far before we stopped developing, but we still ended up learning a lot.

Right now, all there is in the master branch is a keyboard that acts a a [Janko keyboard](https://en.wikipedia.org/wiki/Jank%C3%B3_keyboard), in the simple_keyboard folder, and code that playbacks a simple hard-coded melody in playback_melody. You can run either by going into the folder and running `cargo run --release`.

But even for that we needed to design the structures for out notes and melodies and the interface of the synth. They can be found in the melody and synth_interface folders respectively.

You can also switch to the simple_sequencer branch and see something that resembles a sequencer more in the simple_sequencer folder.
