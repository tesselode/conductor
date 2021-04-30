mod backend;
mod ctx;
mod error;

use std::sync::Arc;

use assert_no_alloc::assert_no_alloc;
use basedrop::{Collector, Owned};
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};
use ringbuf::{Producer, RingBuffer};

use crate::{
	command::{Command, SoundCommand},
	sound::{data::SoundData, Sound, SoundId},
};

use self::{
	backend::Backend,
	ctx::AudioContext,
	error::{CommandQueueFullError, SetupError},
};

pub struct AudioManagerSettings {
	pub num_commands: usize,
	pub num_sounds: usize,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self {
			num_commands: 100,
			num_sounds: 100,
		}
	}
}

pub struct AudioManager {
	ctx: Arc<AudioContext>,
	command_producer: Producer<Command>,
	collector: Collector,
	_stream: Stream,
}

impl AudioManager {
	pub fn new(settings: AudioManagerSettings) -> Result<Self, SetupError> {
		let (command_producer, command_consumer) = RingBuffer::new(settings.num_commands).split();
		let host = cpal::default_host();
		let device = host
			.default_output_device()
			.ok_or(SetupError::NoDefaultOutputDevice)?;
		let config = device.default_output_config()?.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let ctx = Arc::new(AudioContext::new(sample_rate));
		let mut backend = Backend::new(ctx.clone(), command_consumer, settings);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				for frame in data.chunks_exact_mut(channels as usize) {
					let out = assert_no_alloc(|| backend.process());
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
			ctx,
			command_producer,
			collector: Collector::new(),
			_stream: stream,
		})
	}

	pub fn add_sound(
		&mut self,
		data: impl SoundData + 'static,
	) -> Result<SoundId, CommandQueueFullError> {
		let id = SoundId::new();
		let sound = Owned::new(&self.collector.handle(), Sound::new(Arc::new(data)));
		self.command_producer
			.push(Command::Sound(SoundCommand::AddSound { id, sound }))
			.map_err(|_| CommandQueueFullError)?;
		Ok(id)
	}

	pub fn remove_sound(&mut self, id: SoundId) -> Result<(), CommandQueueFullError> {
		self.command_producer
			.push(Command::Sound(SoundCommand::RemoveSound { id }))
			.map_err(|_| CommandQueueFullError)
	}

	pub fn free_unused_resources(&mut self) {
		self.collector.collect();
	}
}
