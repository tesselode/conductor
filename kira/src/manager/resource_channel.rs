use ringbuf::{Consumer, Producer, RingBuffer};

use crate::sound::{instance::Instance, Sound};

use super::AudioManagerSettings;

pub(crate) struct NewResourceProducers {
	pub sound_producer: Producer<Sound>,
	pub instance_producer: Producer<Instance>,
}

pub(crate) struct NewResourceConsumers {
	pub sound_consumer: Consumer<Sound>,
	pub instance_consumer: Consumer<Instance>,
}

pub(crate) fn create_new_resource_channels(
	settings: &AudioManagerSettings,
) -> (NewResourceProducers, NewResourceConsumers) {
	let (sound_producer, sound_consumer) = RingBuffer::new(settings.num_sounds).split();
	let (instance_producer, instance_consumer) = RingBuffer::new(settings.num_instances).split();
	(
		NewResourceProducers {
			sound_producer,
			instance_producer,
		},
		NewResourceConsumers {
			sound_consumer,
			instance_consumer,
		},
	)
}

pub(crate) struct UnusedResourceProducers {
	pub sound_producer: Producer<Sound>,
}

pub(crate) struct UnusedResourceConsumers {
	pub sound_consumer: Consumer<Sound>,
}

pub(crate) fn create_unused_resource_channels(
	settings: &AudioManagerSettings,
) -> (UnusedResourceProducers, UnusedResourceConsumers) {
	let (sound_producer, sound_consumer) = RingBuffer::new(settings.num_sounds).split();
	(
		UnusedResourceProducers { sound_producer },
		UnusedResourceConsumers { sound_consumer },
	)
}
