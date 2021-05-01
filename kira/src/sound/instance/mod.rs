pub mod handle;

use std::sync::atomic::AtomicUsize;

use atomig::Ordering;

use self::handle::InstanceHandle;

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
}

impl Instance {
	pub fn new() -> Self {
		Self {
			state: InstanceState::Playing,
			playback_position: 0.0,
		}
	}

	pub fn state(&self) -> InstanceState {
		self.state
	}

	pub fn playback_position(&self) -> f64 {
		self.playback_position
	}

	pub fn update(&mut self, dt: f64, duration: f64) {
		if self.state == InstanceState::Playing {
			self.playback_position += dt;
			if self.playback_position > duration {
				self.state = InstanceState::Stopped;
			}
		}
	}
}
