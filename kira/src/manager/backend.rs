use llq::Node;

use crate::{frame::Frame, sound::Sound};

use super::{
	resource_channel::{ResourceConsumers, ResourceProducers},
	AudioManagerSettings,
};

pub struct Backend {
	sample_rate: u32,
	dt: f64,
	new_resource_consumers: ResourceConsumers,
	unused_resource_producers: ResourceProducers,
	sounds: Vec<Node<Sound>>,
}

impl Backend {
	pub fn new(
		settings: AudioManagerSettings,
		sample_rate: u32,
		new_resource_consumers: ResourceConsumers,
		unused_resource_producers: ResourceProducers,
	) -> Self {
		Self {
			sample_rate,
			dt: 1.0 / sample_rate as f64,
			new_resource_consumers,
			unused_resource_producers,
			sounds: Vec::with_capacity(settings.num_sounds),
		}
	}

	pub fn receive_resources(&mut self) {
		while let Some(sound) = self.new_resource_consumers.sound_consumer.pop() {
			if self.sounds.len() < self.sounds.capacity() {
				self.sounds.push(sound);
			} else {
				self.unused_resource_producers.sound_producer.push(sound);
			}
		}
	}

	pub fn process(&mut self) -> Frame {
		Frame::from_mono(0.0)
	}
}
