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

pub struct InstanceSettings {
	pub volume: Value<f64>,
	pub playback_rate: Value<f64>,
	pub panning: Value<f64>,
	pub loop_start: InstanceLoopStart,
}

impl InstanceSettings {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn volume(self, volume: impl Into<Value<f64>>) -> Self {
		Self {
			volume: volume.into(),
			..self
		}
	}

	pub fn playback_rate(self, playback_rate: impl Into<Value<f64>>) -> Self {
		Self {
			playback_rate: playback_rate.into(),
			..self
		}
	}

	pub fn panning(self, panning: impl Into<Value<f64>>) -> Self {
		Self {
			panning: panning.into(),
			..self
		}
	}

	pub fn loop_start(self, loop_start: impl Into<InstanceLoopStart>) -> Self {
		Self {
			loop_start: loop_start.into(),
			..self
		}
	}

	pub(crate) fn into_internal(self, sound: &Sound) -> InternalInstanceSettings {
		InternalInstanceSettings {
			volume: self.volume,
			playback_rate: self.playback_rate,
			panning: self.panning,
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
			loop_start: InstanceLoopStart::DefaultForSound,
		}
	}
}

pub struct InternalInstanceSettings {
	pub volume: Value<f64>,
	pub playback_rate: Value<f64>,
	pub panning: Value<f64>,
	pub loop_start: Option<f64>,
}
