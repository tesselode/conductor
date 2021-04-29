pub mod arrangement;
pub mod static_sound;

use crate::Frame;

pub trait SoundData: Send + Sync {
	fn duration(&self) -> f64;

	fn frame_at_position(&self, position: f64) -> Frame;
}
