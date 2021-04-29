use basedrop::Owned;

use crate::{
	command::ResourceCommand,
	sound::{Sound, SoundId},
	static_container::index_map::StaticIndexMap,
	Frame,
};

pub(crate) struct Sounds {
	sounds: StaticIndexMap<SoundId, Owned<Sound>>,
}

impl Sounds {
	pub fn new(capacity: usize) -> Self {
		Self {
			sounds: StaticIndexMap::new(capacity),
		}
	}

	pub fn sound(&self, id: SoundId) -> Option<&Owned<Sound>> {
		self.sounds.get(&id)
	}

	pub fn sound_mut(&mut self, id: SoundId) -> Option<&mut Owned<Sound>> {
		self.sounds.get_mut(&id)
	}

	pub fn frame_at_position(&self, id: SoundId, position: f64) -> Option<Frame> {
		self.sound(id)
			.map(|sound| sound.frame_at_position(position))
	}

	pub fn run_command(&mut self, command: ResourceCommand) {
		match command {
			ResourceCommand::AddSound(sound) => {
				self.sounds.try_insert(sound.id(), sound).ok();
			}
			ResourceCommand::RemoveSound(id) => {
				self.sounds.remove(&id);
			}
		}
	}

	pub fn update(&mut self, dt: f64) {
		for (_, sound) in &mut self.sounds {
			sound.update_cooldown(dt);
		}
	}
}
