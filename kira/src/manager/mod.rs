mod backend;
mod command;
mod ctx;
mod error;

use std::sync::Arc;

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};
use ringbuf::{Producer, RingBuffer};

use self::{backend::Backend, command::Command, ctx::AudioContext, error::SetupError};

pub struct AudioManagerSettings {
	pub num_commands: usize,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self { num_commands: 100 }
	}
}

pub struct AudioManager {
	ctx: Arc<AudioContext>,
	command_producer: Producer<Command>,
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
		let mut backend = Backend::new(ctx.clone(), command_consumer);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				for frame in data.chunks_exact_mut(channels as usize) {
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
			ctx,
			command_producer,
			_stream: stream,
		})
	}
}
