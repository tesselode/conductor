use crate::{
	frame::Frame,
	sound::instance::{Instance, InstanceState},
};

pub(crate) struct Instances {
	instances: Vec<Instance>,
}

impl Instances {
	pub fn new(capacity: usize) -> Self {
		Self {
			instances: Vec::with_capacity(capacity),
		}
	}

	pub fn push(&mut self, instance: Instance) {
		if self.instances.len() >= self.instances.capacity() {
			self.instances.remove(0);
		}
		self.instances.push(instance);
	}

	pub fn process(&mut self, dt: f64) -> Frame {
		let out = self
			.instances
			.iter_mut()
			.fold(Frame::from_mono(0.0), |out, instance| {
				out + instance.process(dt)
			});
		self.instances
			.retain(|instance| instance.state() != InstanceState::Stopped);
		out
	}
}
