pub mod producer;

use basedrop::Owned;

use crate::sound::{
	instance::{Instance, InstanceId},
	Sound, SoundId,
};

pub(crate) enum SoundCommand {
	AddSound {
		id: SoundId,
		sound: Owned<Sound>,
	},
	RemoveSound {
		id: SoundId,
	},
	AddInstance {
		sound_id: SoundId,
		instance_id: InstanceId,
		instance: Instance,
	},
}

pub(crate) enum Command {
	Sound(SoundCommand),
}
