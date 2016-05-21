trait Synth {
    // TODO make sure all the appropriate functions work well in real-time situation

    //fn get_synth_name() -> String;
    
    fn get_audio_frame() -> (f32,f32);
        // The tuple of f32 represents the left and right output
        // TODO in the grand scheme of things this shouldn't be hardcoded

    fn get_number_of_synth_params() -> u32;
    fn get_synth_param_name(param_id: u32) -> String;
    fn set_synth_param(param_id: u32, value: f64);
    fn get_synth_param(param_id: u32) -> f64;
    fn set_all_synth_params(synth_params: Vec<Option<f64>>);
        // TODO consider whether a None in the array sets the param to default or doesn't change it
        // or whether we need this function in the first place

    fn get_number_of_note_params() -> u32;
    fn get_note_param_label(param_id: u32) -> String;
    fn set_note_param(note_id: u32, param_id: u32, value: f64);
    fn get_note_param(note_id: u32, param_id: u32) -> Option<f64>;
        // The reason an Option is used is because the note with the note_id might not exist.
    fn set_all_note_params(note_id: u32, note_params: Vec<Option<f64>>);
        // TODO consider whether a None in the array sets the param to default or doesn't change it
    fn get_all_note_params(note_id: u32) -> Option<Vec<f64>>;

    fn note_on(note_id: u32, delay: u32, note_params: Vec<Option<f64>>);
        // delay is used to specify if the note is supposed to start playing midway
        // if a note param is None, then it is set to default
    fn note_off(note_id: u32);

    fn all_notes_off();
    fn silence(); // like all_notes_off, but makes sure everything stops playing
}
    
