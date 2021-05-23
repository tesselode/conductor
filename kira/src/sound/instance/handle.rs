use std::sync::{atomic::Ordering, Arc};

use super::{InstanceController, InstanceState};

pub struct InstanceHandle {
	controller: Arc<InstanceController>,
}

impl InstanceHandle {
	pub(crate) fn new(controller: Arc<InstanceController>) -> Self {
		Self { controller }
	}

	pub fn stop(&self) {
		self.controller
			.state
			.store(InstanceState::Stopped as u8, Ordering::SeqCst);
	}
}
