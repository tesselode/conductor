use crate::{sound::Sound, value::Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstanceLoopStart {
	DefaultForSound,
	Custom(f64),
	None,
}

impl InstanceLoopStart {
	pub fn into_option(self, default_loop_start: Option<f64>) -> Option<f64> {
		match self {
			InstanceLoopStart::DefaultForSound => default_loop_start,
			InstanceLoopStart::Custom(loop_start) => Some(loop_start),
			InstanceLoopStart::None => None,
		}
	}
}

impl From<f64> for InstanceLoopStart {
	fn from(loop_start: f64) -> Self {
		Self::Custom(loop_start)
	}
}

impl Default for InstanceLoopStart {
	fn default() -> Self {
		Self::DefaultForSound
	}
}

pub struct InstanceSettings {
	/// The volume of the instance.
	pub volume: Value<f64>,
	/// The playback rate of the instance, as a factor of the original
	/// playback rate.
	pub playback_rate: Value<f64>,
	/// The panning of the instance (0 = hard left, 1 = hard right).
	pub panning: Value<f64>,
	/// The position to start playing the instance at (in seconds).
	pub start_position: f64,
	/// Whether to play the instance in reverse.
	pub reverse: bool,
	/// Whether the instance should loop, and if so, the position
	/// it should jump back to when it reaches the end.
	pub loop_start: InstanceLoopStart,
}

impl InstanceSettings {
	pub fn new() -> Self {
		Self::default()
	}

	/// Sets the volume of the instance.
	pub fn volume<V: Into<Value<f64>>>(self, volume: V) -> Self {
		Self {
			volume: volume.into(),
			..self
		}
	}

	/// Sets the playback rate of the instance.
	pub fn playback_rate<P: Into<Value<f64>>>(self, playback_rate: P) -> Self {
		Self {
			playback_rate: playback_rate.into(),
			..self
		}
	}

	/// Sets the panning of the instance.
	pub fn panning<P: Into<Value<f64>>>(self, panning: P) -> Self {
		Self {
			panning: panning.into(),
			..self
		}
	}

	/// Sets where in the sound playback will start (in seconds).
	pub fn start_position(self, start_position: f64) -> Self {
		Self {
			start_position,
			..self
		}
	}

	/// Play the instance in reverse.
	pub fn reverse(self, reverse: bool) -> Self {
		Self { reverse, ..self }
	}

	/// Sets the portion of the sound that should be looped.
	pub fn loop_start<S: Into<InstanceLoopStart>>(self, start: S) -> Self {
		Self {
			loop_start: start.into(),
			..self
		}
	}

	pub(crate) fn into_internal(self, sound: &Sound) -> InternalInstanceSettings {
		InternalInstanceSettings {
			volume: self.volume,
			playback_rate: self.playback_rate,
			panning: self.panning,
			start_position: if self.reverse {
				sound.data().duration() - self.start_position
			} else {
				self.start_position
			},
			reverse: self.reverse,
			loop_start: self.loop_start.into_option(sound.default_loop_start()),
		}
	}
}

impl Default for InstanceSettings {
	fn default() -> Self {
		Self {
			volume: Value::Fixed(1.0),
			playback_rate: Value::Fixed(1.0),
			panning: Value::Fixed(0.5),
			start_position: 0.0,
			reverse: false,
			loop_start: InstanceLoopStart::default(),
		}
	}
}

pub struct InternalInstanceSettings {
	pub volume: Value<f64>,
	pub playback_rate: Value<f64>,
	pub panning: Value<f64>,
	pub start_position: f64,
	pub reverse: bool,
	pub loop_start: Option<f64>,
}
