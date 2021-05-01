pub mod data;
pub mod handle;
pub mod instance;
pub(crate) mod sounds;

use std::sync::{atomic::AtomicUsize, Arc};

use atomig::Ordering;

use crate::{
	frame::Frame, parameter::parameters::Parameters, static_container::index_map::StaticIndexMap,
};

use self::{
	data::SoundData,
	handle::SoundHandle,
	instance::{settings::InstanceSettings, Instance, InstanceId, InstanceState},
};

static NEXT_SOUND_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundId(usize);

impl SoundId {
	pub(crate) fn new() -> Self {
		Self(NEXT_SOUND_INDEX.fetch_add(1, Ordering::SeqCst))
	}
}

impl From<&SoundHandle> for SoundId {
	fn from(handle: &SoundHandle) -> Self {
		handle.id()
	}
}

pub struct SoundSettings {
	pub num_instances: usize,
	pub default_loop_start: Option<f64>,
}

impl SoundSettings {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn num_instances(self, num_instances: usize) -> Self {
		Self {
			num_instances,
			..self
		}
	}

	pub fn default_loop_start(self, default_loop_start: impl Into<Option<f64>>) -> Self {
		Self {
			default_loop_start: default_loop_start.into(),
			..self
		}
	}
}

impl Default for SoundSettings {
	fn default() -> Self {
		Self {
			num_instances: 10,
			default_loop_start: None,
		}
	}
}

pub(crate) struct Sound {
	data: Arc<dyn SoundData>,
	instances: StaticIndexMap<InstanceId, Instance>,
	default_loop_start: Option<f64>,
}

impl Sound {
	pub fn new(data: Arc<dyn SoundData>, settings: SoundSettings) -> Self {
		Self {
			data,
			instances: StaticIndexMap::new(settings.num_instances),
			default_loop_start: settings.default_loop_start,
		}
	}

	pub fn data(&self) -> &Arc<dyn SoundData> {
		&self.data
	}

	pub fn default_loop_start(&self) -> Option<f64> {
		self.default_loop_start
	}

	pub fn add_instance(&mut self, instance_id: InstanceId, settings: InstanceSettings) {
		self.instances
			.try_insert(instance_id, Instance::new(settings.into_internal(&self)))
			.ok();
	}

	pub fn process(&mut self, dt: f64, parameters: &Parameters) -> Frame {
		let mut out = Frame::from_mono(0.0);
		for i in (0..self.instances.len()).rev() {
			if let Some((_, instance)) = self.instances.get_index_mut(i) {
				out += self
					.data
					.frame_at_position(instance.playback_position())
					.panned(instance.panning() as f32)
					* instance.volume() as f32;
				if instance.state() == InstanceState::Stopped {
					self.instances.shift_remove_index(i);
				} else {
					instance.update(dt, self.data.duration(), parameters);
				}
			}
		}
		out
	}
}
