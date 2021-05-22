use llq::Queue;

use crate::sound::Sound;

pub struct ResourceProducers {
	pub sound_producer: llq::Producer<Sound>,
}

pub struct ResourceConsumers {
	pub sound_consumer: llq::Consumer<Sound>,
}

pub fn create_resource_channels() -> (ResourceProducers, ResourceConsumers) {
	let (sound_producer, sound_consumer) = Queue::new().split();
	(
		ResourceProducers { sound_producer },
		ResourceConsumers { sound_consumer },
	)
}
