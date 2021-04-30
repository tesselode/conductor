use basedrop::Owned;

use crate::sound::{Sound, SoundId};

pub enum SoundCommand {
	AddSound { id: SoundId, sound: Owned<Sound> },
	RemoveSound { id: SoundId },
}

pub enum Command {
	Sound(SoundCommand),
}
