use std::sync::Arc;

use atomig::Ordering;
use ringbuf::Consumer;

use crate::{
	command::Command, frame::Frame, parameter::parameters::Parameters, sound::sounds::Sounds,
};

use super::{ctx::AudioContext, AudioManagerSettings};

pub(crate) struct Backend {
	ctx: Arc<AudioContext>,
	dt: f64,
	command_consumer: Consumer<Command>,
	sounds: Sounds,
	parameters: Parameters,
}

impl Backend {
	pub fn new(
		ctx: Arc<AudioContext>,
		command_consumer: Consumer<Command>,
		settings: AudioManagerSettings,
	) -> Self {
		let dt = 1.0 / ctx.sample_rate.load(Ordering::SeqCst) as f64;
		Self {
			ctx,
			dt,
			command_consumer,
			sounds: Sounds::new(settings.num_sounds),
			parameters: Parameters::new(settings.num_parameters),
		}
	}

	pub fn process(&mut self) -> Frame {
		while let Some(command) = self.command_consumer.pop() {
			match command {
				Command::Sound(command) => {
					self.sounds.run_command(command);
				}
				Command::Parameter(command) => {
					self.parameters.run_command(command);
				}
			}
		}
		self.parameters.update(self.dt);
		self.sounds.process(self.dt, &self.parameters)
	}
}
