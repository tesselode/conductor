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
use ringbuf::RingBuffer;

use crate::{
	command::{
		producer::{CommandError, CommandProducer},
		Command, ParameterCommand, SoundCommand,
	},
	parameter::{handle::ParameterHandle, Parameter, ParameterId},
	sound::{data::SoundData, handle::SoundHandle, Sound, SoundId, SoundSettings},
};

use self::{backend::Backend, ctx::AudioContext, error::SetupError};

pub struct AudioManagerSettings {
	pub num_commands: usize,
	pub num_sounds: usize,
	pub num_parameters: usize,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self {
			num_commands: 100,
			num_sounds: 100,
			num_parameters: 25,
		}
	}
}

pub struct AudioManager {
	ctx: Arc<AudioContext>,
	command_producer: CommandProducer,
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
			command_producer: CommandProducer::new(command_producer),
			collector: Collector::new(),
			_stream: stream,
		})
	}

	pub fn add_sound(
		&mut self,
		data: impl SoundData + 'static,
		settings: SoundSettings,
	) -> Result<SoundHandle, CommandError> {
		let id = SoundId::new();
		let sound = Owned::new(
			&self.collector.handle(),
			Sound::new(Arc::new(data), settings),
		);
		self.command_producer
			.push(Command::Sound(SoundCommand::AddSound { id, sound }))?;
		let handle = SoundHandle::new(id, self.command_producer.clone());
		Ok(handle)
	}

	pub fn remove_sound(&mut self, id: impl Into<SoundId>) -> Result<(), CommandError> {
		self.command_producer
			.push(Command::Sound(SoundCommand::RemoveSound { id: id.into() }))
	}

	pub fn add_parameter(&mut self, starting_value: f64) -> Result<ParameterHandle, CommandError> {
		let id = ParameterId::new();
		self.command_producer
			.push(Command::Parameter(ParameterCommand::AddParameter {
				id,
				starting_value,
			}))?;
		let handle = ParameterHandle::new(id, self.command_producer.clone());
		Ok(handle)
	}

	pub fn remove_parameter(&mut self, id: impl Into<ParameterId>) -> Result<(), CommandError> {
		self.command_producer
			.push(Command::Parameter(ParameterCommand::RemoveParameter {
				id: id.into(),
			}))
	}

	pub fn free_unused_resources(&mut self) {
		self.collector.collect();
	}
}
