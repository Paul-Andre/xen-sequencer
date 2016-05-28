#[derive(Debug)]
pub enum SynthEvent {
    On{ note_id: u32, note_params: Vec<Option<f64>> },
    Off{ note_id: u32 },
}
