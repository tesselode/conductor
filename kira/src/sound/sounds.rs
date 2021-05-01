use basedrop::Owned;

use crate::{
	command::SoundCommand, frame::Frame, parameter::parameters::Parameters,
	static_container::index_map::StaticIndexMap,
};

use super::{Sound, SoundId};

pub(crate) struct Sounds {
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
			SoundCommand::AddInstance {
				sound_id,
				instance_id,
				instance,
			} => {
				if let Some(sound) = self.sounds.get_mut(&sound_id) {
					sound.add_instance(instance_id, instance);
				}
			}
		}
	}

	pub fn process(&mut self, dt: f64, parameters: &Parameters) -> Frame {
		self.sounds
			.iter_mut()
			.fold(Frame::from_mono(0.0), |previous, (_, sound)| {
				previous + sound.process(dt, parameters)
			})
	}
}
