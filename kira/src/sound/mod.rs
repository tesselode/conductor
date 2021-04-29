//! A chunk of audio data.

pub mod data;
pub mod handle;
mod id;
mod settings;

use std::sync::Arc;

pub use id::SoundId;
pub use settings::SoundSettings;

use crate::{
	frame::Frame,
	group::{groups::Groups, GroupId, GroupSet},
	mixer::TrackIndex,
};

use self::data::SoundData;

/// A piece of audio that can be played by an [`AudioManager`](crate::manager::AudioManager).
pub(crate) struct Sound {
	id: SoundId,
	data: Arc<dyn SoundData>,
	default_track: TrackIndex,
	cooldown: Option<f64>,
	semantic_duration: Option<f64>,
	default_loop_start: Option<f64>,
	groups: GroupSet,
	cooldown_timer: f64,
}

impl Sound {
	pub fn new(data: impl SoundData + 'static, settings: SoundSettings) -> Self {
		Self {
			id: settings.id.unwrap_or(SoundId::new()),
			data: Arc::new(data),
			default_track: settings.default_track,
			cooldown: settings.cooldown,
			semantic_duration: settings.semantic_duration,
			default_loop_start: settings.default_loop_start,
			groups: settings.groups,
			cooldown_timer: 0.0,
		}
	}

	/// Gets the unique identifier for this sound.
	pub fn id(&self) -> SoundId {
		self.id
	}

	/// Gets this sound's data.
	pub fn data(&self) -> Arc<dyn SoundData> {
		self.data.clone()
	}

	/// Gets the default track instances of this sound will play on.
	pub fn default_track(&self) -> TrackIndex {
		self.default_track
	}

	/// Gets the groups this sound belongs to.
	pub fn groups(&self) -> &GroupSet {
		&self.groups
	}

	/// Gets the duration of the sound (in seconds).
	pub fn duration(&self) -> f64 {
		self.data.duration()
	}

	/// Gets the "musical length" of the sound (if there is one).
	pub fn semantic_duration(&self) -> Option<f64> {
		self.semantic_duration
	}

	/// Returns the default time (in seconds) instances
	/// of this sound will loop back to when they reach
	/// the end.
	pub fn default_loop_start(&self) -> Option<f64> {
		self.default_loop_start
	}

	/// Gets the frame of this sound at an arbitrary time
	/// in seconds, interpolating between samples if necessary.
	pub fn frame_at_position(&self, position: f64) -> Frame {
		self.data.frame_at_position(position)
	}

	/// Starts the cooldown timer for the sound.
	pub(crate) fn start_cooldown(&mut self) {
		if let Some(cooldown) = self.cooldown {
			self.cooldown_timer = cooldown;
		}
	}

	/// Updates the cooldown timer for the sound.
	pub(crate) fn update_cooldown(&mut self, dt: f64) {
		if self.cooldown_timer > 0.0 {
			self.cooldown_timer -= dt;
		}
	}

	/// Gets whether the sound is currently "cooling down".
	///
	/// If it is, a new instance of the sound should not
	/// be started until the timer is up.
	pub(crate) fn cooling_down(&self) -> bool {
		self.cooldown_timer > 0.0
	}

	/// Returns if this sound is in the group with the given ID.
	pub(crate) fn is_in_group(&self, id: GroupId, all_groups: &Groups) -> bool {
		self.groups.has_ancestor(id, all_groups)
	}
}
