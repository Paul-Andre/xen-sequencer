use note::Note;
use synth_event::SynthEvent;
use tagged_event::TaggedEvent;

pub struct Melody {
    pub notes: Vec<Note>,
}

impl Melody {
    pub fn get_event_list(&self, frame_rate: f64, starting_frame: u32) -> Vec<TaggedEvent<SynthEvent>> {
        let mut ret: Vec<TaggedEvent<SynthEvent>> = Vec::with_capacity(self.notes.len()*2);
        for (i, note) in self.notes.iter().enumerate() {
            ret.push(TaggedEvent {
                tag: starting_frame + (frame_rate * note.start) as u32,
                event: SynthEvent::On {
                    note_id: i as u32,
                    note_params: vec![Some(note.amplitude), Some(note.pitch.get_frequency())],
                },
            });
            ret.push(TaggedEvent {
                tag: starting_frame + (frame_rate * (note.start+note.duration)) as u32,
                event: SynthEvent::Off {
                    note_id: i as u32,
                },
            });
        }
        ret.sort();
        return ret;
    }
}

