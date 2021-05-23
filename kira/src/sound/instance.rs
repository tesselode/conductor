pub mod handle;

use std::sync::{
	atomic::{AtomicU8, Ordering},
	Arc,
};

use crate::frame::Frame;

use super::data::SoundData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceState {
	Playing,
	Stopped,
}

impl From<u8> for InstanceState {
	fn from(index: u8) -> Self {
		match index {
			0 => Self::Playing,
			1 => Self::Stopped,
			_ => panic!("Invalid InstanceState"),
		}
	}
}

pub(crate) struct InstanceController {
	state: AtomicU8,
}

impl InstanceController {
	pub fn new() -> Self {
		Self {
			state: AtomicU8::new(InstanceState::Playing as u8),
		}
	}
}

pub(crate) struct Instance {
	data: Arc<dyn SoundData>,
	controller: Option<Arc<InstanceController>>,
	state: InstanceState,
	position: f64,
}

impl Instance {
	pub fn new(data: Arc<dyn SoundData>, controller: Arc<InstanceController>) -> Self {
		let state = controller.state.load(Ordering::SeqCst).into();
		Self {
			data,
			controller: Some(controller),
			state,
			position: 0.0,
		}
	}

	pub fn state(&self) -> InstanceState {
		self.state
	}

	pub fn update_from_controller(&mut self) {
		if let Some(controller) = &self.controller {
			self.state = controller.state.load(Ordering::SeqCst).into();
		}
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

	pub fn save_state_to_controller(&mut self) {
		if let Some(controller) = &self.controller {
			controller.state.store(self.state as u8, Ordering::SeqCst);
		}
	}
}
