use crate::{
	command::InstanceCommand,
	group::groups::Groups,
	instance::{Instance, InstanceId, StopInstanceSettings},
	parameter::Parameters,
	sound::SoundId,
	sounds::Sounds,
	static_container::{index_map::StaticIndexMap, vec::StaticVec},
};

use super::mixer::Mixer;

pub(crate) struct Instances {
	instances: StaticIndexMap<InstanceId, Instance>,
	instances_to_remove: StaticVec<InstanceId>,
}

impl Instances {
	pub fn new(capacity: usize) -> Self {
		Self {
			instances: StaticIndexMap::new(capacity),
			instances_to_remove: StaticVec::new(capacity),
		}
	}

	pub fn stop_instances_of(&mut self, id: SoundId, settings: StopInstanceSettings) {
		for (_, instance) in &mut self.instances {
			if instance.sound_id() == id {
				instance.stop(settings);
			}
		}
	}

	pub fn run_command(
		&mut self,
		command: InstanceCommand,
		sounds: &mut Sounds,
		all_groups: &Groups,
	) {
		match command {
			InstanceCommand::Play(instance_id, instance) => {
				if let Some(sound) = sounds.sound_mut(instance.sound_id()) {
					if !sound.cooling_down() {
						// if we're at the instance limit, remove the instance that was
						// started the longest time ago.
						if self.instances.len() >= self.instances.capacity() {
							self.instances.shift_remove_index(0);
						}
						self.instances.try_insert(instance_id, instance).ok();
						sound.start_cooldown();
					}
				}
			}
			InstanceCommand::SetInstanceVolume(id, value) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.set_volume(value);
				}
			}
			InstanceCommand::SetInstancePlaybackRate(id, value) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.set_playback_rate(value);
				}
			}
			InstanceCommand::SetInstancePanning(id, value) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.set_panning(value);
				}
			}
			InstanceCommand::SeekInstance(id, offset) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.seek(offset);
				}
			}
			InstanceCommand::SeekInstanceTo(id, position) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.seek_to(position);
				}
			}
			InstanceCommand::PauseInstance(id, settings) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.pause(settings);
				}
			}
			InstanceCommand::ResumeInstance(id, settings) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.resume(settings);
				}
			}
			InstanceCommand::StopInstance(id, settings) => {
				if let Some(instance) = self.instances.get_mut(&id) {
					instance.stop(settings);
				}
			}
			InstanceCommand::PauseInstancesOf(id, settings) => {
				for (_, instance) in &mut self.instances {
					if instance.sound_id() == id {
						instance.pause(settings);
					}
				}
			}
			InstanceCommand::ResumeInstancesOf(id, settings) => {
				for (_, instance) in &mut self.instances {
					if instance.sound_id() == id {
						instance.resume(settings);
					}
				}
			}
			InstanceCommand::StopInstancesOf(id, settings) => {
				self.stop_instances_of(id, settings);
			}
			InstanceCommand::PauseGroup(id, settings) => {
				for (_, instance) in &mut self.instances {
					if let Some(sound) = sounds.sound(instance.sound_id()) {
						if sound.is_in_group(id, all_groups) {
							instance.pause(settings);
						}
					}
				}
			}
			InstanceCommand::ResumeGroup(id, settings) => {
				for (_, instance) in &mut self.instances {
					if let Some(sound) = sounds.sound(instance.sound_id()) {
						if sound.is_in_group(id, all_groups) {
							instance.resume(settings);
						}
					}
				}
			}
			InstanceCommand::StopGroup(id, settings) => {
				for (_, instance) in &mut self.instances {
					if let Some(sound) = sounds.sound(instance.sound_id()) {
						if sound.is_in_group(id, all_groups) {
							instance.stop(settings);
						}
					}
				}
			}
			InstanceCommand::PauseInstancesOfSequence(id, settings) => {
				for (_, instance) in &mut self.instances {
					if instance.sequence_id() == Some(id) {
						instance.pause(settings);
					}
				}
			}
			InstanceCommand::ResumeInstancesOfSequence(id, settings) => {
				for (_, instance) in &mut self.instances {
					if instance.sequence_id() == Some(id) {
						instance.resume(settings);
					}
				}
			}
			InstanceCommand::StopInstancesOfSequence(id, settings) => {
				for (_, instance) in &mut self.instances {
					if instance.sequence_id() == Some(id) {
						instance.stop(settings);
					}
				}
			}
		}
	}

	pub fn process(
		&mut self,
		dt: f64,
		sounds: &Sounds,
		mixer: &mut Mixer,
		parameters: &Parameters,
	) {
		// TODO: simplify this code (preferably by removing self.instances_to_remove)
		// while making sure every sample of the sound gets played before the instance is removed
		for (instance_id, instance) in &mut self.instances {
			if instance.playing() {
				mixer.add_input(instance.track_index(), instance.get_sample(sounds));
			}
			if instance.finished() {
				self.instances_to_remove.try_push(*instance_id).ok();
			}
			instance.update(dt, parameters);
		}
		for instance_id in self.instances_to_remove.drain(..) {
			self.instances.shift_remove(&instance_id);
		}
	}
}
