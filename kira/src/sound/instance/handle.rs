use crate::{command::producer::CommandProducer, sound::SoundId};

use super::InstanceId;

pub struct InstanceHandle {
	id: InstanceId,
	sound_id: SoundId,
	command_producer: CommandProducer,
}

impl InstanceHandle {
	pub(crate) fn new(
		id: InstanceId,
		sound_id: SoundId,
		command_producer: CommandProducer,
	) -> Self {
		Self {
			id,
			sound_id,
			command_producer,
		}
	}

	pub fn id(&self) -> InstanceId {
		self.id
	}

	pub fn sound_id(&self) -> SoundId {
		self.sound_id
	}
}
