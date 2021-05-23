use ringbuf::{Consumer, Producer, RingBuffer};

use crate::sound::Sound;

use super::AudioManagerSettings;

pub struct ResourceProducers {
	pub sound_producer: Producer<Sound>,
}

pub struct ResourceConsumers {
	pub sound_consumer: Consumer<Sound>,
}

pub fn create_resource_channels(
	settings: &AudioManagerSettings,
) -> (ResourceProducers, ResourceConsumers) {
	let (sound_producer, sound_consumer) = RingBuffer::new(settings.num_sounds).split();
	(
		ResourceProducers { sound_producer },
		ResourceConsumers { sound_consumer },
	)
}
