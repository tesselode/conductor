use basedrop::Owned;

use crate::{command::SoundCommand, static_container::index_map::StaticIndexMap};

use super::{Sound, SoundId};

pub struct Sounds {
	sounds: StaticIndexMap<SoundId, Owned<Sound>>,
}

impl Sounds {
	pub fn new(capacity: usize) -> Self {
		Self {
			sounds: StaticIndexMap::new(capacity),
		}
	}

	pub fn run_command(&mut self, command: SoundCommand) {
		match command {
			SoundCommand::AddSound { id, sound } => {
				self.sounds.try_insert(id, sound).ok();
			}
			SoundCommand::RemoveSound { id } => {
				self.sounds.remove(&id);
			}
		}
	}
}
