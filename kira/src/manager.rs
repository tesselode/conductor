mod backend;
mod error;
mod resource_channel;

use std::sync::Arc;

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};

use crate::{
	manager::{backend::Backend, error::SetupError},
	sound::{data::SoundData, handle::SoundHandle, instance::Instance, Sound},
};

use self::{
	error::{AddSoundError, PlaySoundError},
	resource_channel::{
		create_new_resource_channels, create_unused_resource_channels, NewResourceProducers,
		UnusedResourceConsumers,
	},
};

pub struct AudioManagerSettings {
	num_sounds: usize,
	num_instances: usize,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self {
			num_sounds: 100,
			num_instances: 100,
		}
	}
}

pub struct AudioManager {
	_stream: Stream,
	new_resource_producers: NewResourceProducers,
	unused_resource_consumers: UnusedResourceConsumers,
}

impl AudioManager {
	pub fn new(settings: AudioManagerSettings) -> Result<Self, SetupError> {
		let (new_resource_producers, new_resource_consumers) =
			create_new_resource_channels(&settings);
		let (unused_resource_producers, unused_resource_consumers) =
			create_unused_resource_channels(&settings);
		let host = cpal::default_host();
		let device = host
			.default_output_device()
			.ok_or(SetupError::NoDefaultOutputDevice)?;
		let config = device.default_output_config()?.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let mut backend = Backend::new(
			settings,
			sample_rate,
			new_resource_consumers,
			unused_resource_producers,
		);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				#[cfg(feature = "assert_no_alloc")]
				assert_no_alloc::assert_no_alloc(|| backend.receive_resources());
				#[cfg(not(feature = "assert_no_alloc"))]
				backend.receive_resources();
				for frame in data.chunks_exact_mut(channels as usize) {
					#[cfg(feature = "assert_no_alloc")]
					let out = assert_no_alloc::assert_no_alloc(|| backend.process());
					#[cfg(not(feature = "assert_no_alloc"))]
					let out = backend.process();
					if channels == 1 {
						frame[0] = (out.left + out.right) / 2.0;
					} else {
						frame[0] = out.left;
						frame[1] = out.right;
					}
				}
			},
			move |_| {},
		)?;
		stream.play()?;
		Ok(Self {
			_stream: stream,
			new_resource_producers,
			unused_resource_consumers,
		})
	}

	pub fn add_sound(
		&mut self,
		data: impl SoundData + 'static,
	) -> Result<SoundHandle, AddSoundError> {
		let data = Arc::new(data);
		let handle = SoundHandle::new(data.clone());
		self.new_resource_producers
			.sound_producer
			.push(Sound::new(data.clone()))
			.map_err(|_| AddSoundError::SoundLimitReached)?;
		Ok(handle)
	}

	pub fn play(&mut self, sound: &SoundHandle) -> Result<(), PlaySoundError> {
		let data = sound.data().clone();
		self.new_resource_producers
			.instance_producer
			.push(Instance::new(data))
			.map_err(|_| PlaySoundError::InstanceLimitReached)
	}
}
