pub mod handle;
pub mod settings;

use std::sync::atomic::AtomicUsize;

use atomig::Ordering;

use crate::{parameter::parameters::Parameters, value::CachedValue};

use self::{handle::InstanceHandle, settings::InternalInstanceSettings};

static NEXT_INSTANCE_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId(usize);

impl InstanceId {
	pub(crate) fn new() -> Self {
		Self(NEXT_INSTANCE_INDEX.fetch_add(1, Ordering::SeqCst))
	}
}

impl From<&InstanceHandle> for InstanceId {
	fn from(handle: &InstanceHandle) -> Self {
		handle.id()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceState {
	Playing,
	Stopped,
}

pub(crate) struct Instance {
	state: InstanceState,
	playback_position: f64,
	volume: CachedValue<f64>,
	playback_rate: CachedValue<f64>,
	panning: CachedValue<f64>,
	reverse: bool,
	loop_start: Option<f64>,
}

impl Instance {
	pub fn new(settings: InternalInstanceSettings) -> Self {
		Self {
			state: InstanceState::Playing,
			playback_position: settings.start_position,
			volume: CachedValue::new(settings.volume, 1.0),
			playback_rate: CachedValue::new(settings.playback_rate, 1.0),
			panning: CachedValue::new(settings.panning, 0.5).with_valid_range(0.0..1.0),
			reverse: settings.reverse,
			loop_start: settings.loop_start,
		}
	}

	pub fn state(&self) -> InstanceState {
		self.state
	}

	pub fn playback_position(&self) -> f64 {
		self.playback_position
	}

	pub fn volume(&self) -> f64 {
		self.volume.value()
	}

	pub fn panning(&self) -> f64 {
		self.panning.value()
	}

	pub fn update(&mut self, dt: f64, duration: f64, parameters: &Parameters) {
		if self.state == InstanceState::Playing {
			self.volume.update(parameters);
			self.playback_rate.update(parameters);
			self.panning.update(parameters);
			let mut playback_rate = self.playback_rate.value();
			if self.reverse {
				playback_rate *= -1.0;
			}
			self.playback_position += playback_rate * dt;
			if playback_rate < 0.0 {
				if let Some(loop_start) = self.loop_start {
					while self.playback_position < loop_start {
						self.playback_position += duration - loop_start;
					}
				} else if self.playback_position < 0.0 {
					self.state = InstanceState::Stopped;
				}
			} else {
				if let Some(loop_start) = self.loop_start {
					while self.playback_position > duration {
						self.playback_position -= duration - loop_start;
					}
				} else if self.playback_position > duration {
					self.state = InstanceState::Stopped;
				}
			}
		}
	}
}
