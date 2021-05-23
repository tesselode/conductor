use std::sync::Arc;

use crate::frame::Frame;

use super::data::SoundData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceState {
	Playing,
	Stopped,
}

pub(crate) struct Instance {
	data: Arc<dyn SoundData>,
	state: InstanceState,
	position: f64,
}

impl Instance {
	pub fn new(data: Arc<dyn SoundData>) -> Self {
		Self {
			data,
			state: InstanceState::Playing,
			position: 0.0,
		}
	}

	pub fn state(&self) -> InstanceState {
		self.state
	}

	pub fn process(&mut self, dt: f64) -> Frame {
		if self.state == InstanceState::Playing {
			self.position += dt;
			if self.position >= self.data.duration() {
				self.state = InstanceState::Stopped;
			}
			self.data.frame_at_position(self.position)
		} else {
			Frame::from_mono(0.0)
		}
	}
}
