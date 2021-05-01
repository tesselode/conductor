use crate::value::Value;

pub struct InstanceSettings {
	pub volume: Value<f64>,
	pub playback_rate: Value<f64>,
	pub panning: Value<f64>,
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
}

impl Default for InstanceSettings {
	fn default() -> Self {
		Self {
			volume: Value::Fixed(1.0),
			playback_rate: Value::Fixed(1.0),
			panning: Value::Fixed(0.5),
		}
	}
}
