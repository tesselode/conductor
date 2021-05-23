use crate::{
	frame::Frame,
	manager::resource_channel::UnusedResourceProducers,
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

	pub fn update_from_controllers(&mut self) {
		for instance in &mut self.instances {
			instance.update_from_controller();
		}
	}

	pub fn process(
		&mut self,
		dt: f64,
		unused_resource_producers: &mut UnusedResourceProducers,
	) -> Frame {
		let out = self
			.instances
			.iter_mut()
			.fold(Frame::from_mono(0.0), |out, instance| {
				out + instance.process(dt)
			});
		{
			let mut i = 0;
			while i != self.instances.len() {
				if self.instances[i].state() == InstanceState::Stopped {
					unused_resource_producers
						.instance_producer
						.push(self.instances.remove(i))
						.ok();
				} else {
					i += 1;
				}
			}
		}
		self.instances
			.retain(|instance| instance.state() != InstanceState::Stopped);
		out
	}

	pub fn save_state_to_controllers(&mut self) {
		for instance in &mut self.instances {
			instance.save_state_to_controller();
		}
	}
}
