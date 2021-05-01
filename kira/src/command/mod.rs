pub mod producer;

use basedrop::Owned;

use crate::{
	parameter::{tween::Tween, ParameterId},
	sound::{
		instance::{settings::InstanceSettings, InstanceId},
		Sound, SoundId,
	},
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
		settings: InstanceSettings,
	},
}

pub(crate) enum ParameterCommand {
	AddParameter {
		id: ParameterId,
		starting_value: f64,
	},
	RemoveParameter {
		id: ParameterId,
	},
	SetParameter {
		id: ParameterId,
		value: f64,
		tween: Option<Tween>,
	},
}

pub(crate) enum Command {
	Sound(SoundCommand),
	Parameter(ParameterCommand),
}
