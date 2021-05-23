mod instances;

use crate::{frame::Frame, sound::Sound};

use self::instances::Instances;

use super::{
	resource_channel::{NewResourceConsumers, UnusedResourceProducers},
	AudioManagerSettings,
};

pub(crate) struct Backend {
	sample_rate: u32,
	dt: f64,
	new_resource_consumers: NewResourceConsumers,
	unused_resource_producers: UnusedResourceProducers,
	sounds: Vec<Sound>,
	instances: Instances,
}

impl Backend {
	pub fn new(
		settings: AudioManagerSettings,
		sample_rate: u32,
		new_resource_consumers: NewResourceConsumers,
		unused_resource_producers: UnusedResourceProducers,
	) -> Self {
		Self {
			sample_rate,
			dt: 1.0 / sample_rate as f64,
			new_resource_consumers,
			unused_resource_producers,
			sounds: Vec::with_capacity(settings.num_sounds),
			instances: Instances::new(settings.num_instances),
		}
	}

	pub fn receive_resources(&mut self) {
		while let Some(sound) = self.new_resource_consumers.sound_consumer.pop() {
			if self.sounds.len() < self.sounds.capacity() {
				self.sounds.push(sound);
			} else {
				self.unused_resource_producers
					.sound_producer
					.push(sound)
					.ok();
			}
		}

		while let Some(instance) = self.new_resource_consumers.instance_consumer.pop() {
			self.instances.push(instance);
		}
	}

	pub fn process(&mut self) -> Frame {
		self.instances.process(self.dt)
	}
}
