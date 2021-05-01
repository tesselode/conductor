use crate::command::{
	producer::{CommandError, CommandProducer},
	Command, SoundCommand,
};

use super::{
	instance::{handle::InstanceHandle, settings::InstanceSettings, Instance, InstanceId},
	SoundId,
};

#[derive(Clone)]
pub struct SoundHandle {
	id: SoundId,
	command_producer: CommandProducer,
}

impl SoundHandle {
	pub(crate) fn new(id: SoundId, command_producer: CommandProducer) -> Self {
		Self {
			id,
			command_producer,
		}
	}

	pub fn id(&self) -> SoundId {
		self.id
	}

	pub fn play(&mut self, settings: InstanceSettings) -> Result<InstanceHandle, CommandError> {
		let instance_id = InstanceId::new();
		let instance = Instance::new(settings);
		self.command_producer
			.push(Command::Sound(SoundCommand::AddInstance {
				sound_id: self.id,
				instance_id,
				instance,
			}))?;
		let handle = InstanceHandle::new(instance_id, self.id, self.command_producer.clone());
		Ok(handle)
	}
}
