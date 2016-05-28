pub mod note;
pub mod scale_pitch;
pub mod interval;
//mod rythmic_interval;
pub mod tuning;

mod melody; // melody::melody::Melody
// well, this is not something I want.
// and this is extremely ugly either way.
// asldkfj;ldf
pub use melody::Melody;

pub mod synth_event;
pub mod tagged_event;

