use crate::{
	audio_stream::AudioStreamId,
	group::GroupId,
	metronome::MetronomeId,
	mixer::{SendTrackId, SubTrackId},
	parameter::ParameterId,
	sound::SoundId,
};

use indexmap::IndexSet;

use super::{
	error::{
		AddGroupError, AddMetronomeError, AddParameterError, AddSendTrackError, AddSoundError,
		AddStreamError, AddSubTrackError, RemoveGroupError, RemoveMetronomeError,
		RemoveParameterError, RemoveSendTrackError, RemoveSoundError, RemoveStreamError,
		RemoveSubTrackError,
	},
	AudioManagerSettings,
};

pub struct ActiveIds {
	pub active_sound_ids: IndexSet<SoundId>,
	pub active_parameter_ids: IndexSet<ParameterId>,
	pub active_sub_track_ids: IndexSet<SubTrackId>,
	pub active_send_track_ids: IndexSet<SendTrackId>,
	pub active_group_ids: IndexSet<GroupId>,
	pub active_metronome_ids: IndexSet<MetronomeId>,
	pub active_stream_ids: IndexSet<AudioStreamId>,
}

impl ActiveIds {
	pub fn new(settings: &AudioManagerSettings) -> Self {
		Self {
			active_sound_ids: IndexSet::with_capacity(settings.num_sounds),
			active_parameter_ids: IndexSet::with_capacity(settings.num_parameters),
			active_sub_track_ids: IndexSet::with_capacity(settings.num_sub_tracks),
			active_send_track_ids: IndexSet::with_capacity(settings.num_send_tracks),
			active_group_ids: IndexSet::with_capacity(settings.num_groups),
			active_metronome_ids: IndexSet::with_capacity(settings.num_metronomes),
			active_stream_ids: IndexSet::with_capacity(settings.num_streams),
		}
	}

	pub fn add_sound_id(&mut self, id: SoundId) -> Result<(), AddSoundError> {
		if self.active_sound_ids.len() >= self.active_sound_ids.capacity() {
			return Err(AddSoundError::SoundLimitReached);
		}
		self.active_sound_ids.insert(id);
		Ok(())
	}

	pub fn remove_sound_id(&mut self, id: SoundId) -> Result<(), RemoveSoundError> {
		if !self.active_sound_ids.remove(&id) {
			return Err(RemoveSoundError::NoSoundWithId(id));
		}
		Ok(())
	}

	pub fn add_parameter_id(&mut self, id: ParameterId) -> Result<(), AddParameterError> {
		if self.active_parameter_ids.len() >= self.active_parameter_ids.capacity() {
			return Err(AddParameterError::ParameterLimitReached);
		}
		self.active_parameter_ids.insert(id);
		Ok(())
	}

	pub fn remove_parameter_id(&mut self, id: ParameterId) -> Result<(), RemoveParameterError> {
		if !self.active_parameter_ids.remove(&id) {
			return Err(RemoveParameterError::NoParameterWithId(id));
		}
		Ok(())
	}

	pub fn add_sub_track_id(&mut self, id: SubTrackId) -> Result<(), AddSubTrackError> {
		if self.active_sub_track_ids.len() >= self.active_sub_track_ids.capacity() {
			return Err(AddSubTrackError::TrackLimitReached);
		}
		self.active_sub_track_ids.insert(id);
		Ok(())
	}

	pub fn remove_sub_track_id(&mut self, id: SubTrackId) -> Result<(), RemoveSubTrackError> {
		if !self.active_sub_track_ids.remove(&id) {
			return Err(RemoveSubTrackError::NoSubTrackWithId(id));
		}
		Ok(())
	}

	pub fn add_send_track_id(&mut self, id: SendTrackId) -> Result<(), AddSendTrackError> {
		if self.active_send_track_ids.len() >= self.active_send_track_ids.capacity() {
			return Err(AddSendTrackError::TrackLimitReached);
		}
		self.active_send_track_ids.insert(id);
		Ok(())
	}

	pub fn remove_send_track_id(&mut self, id: SendTrackId) -> Result<(), RemoveSendTrackError> {
		if !self.active_send_track_ids.remove(&id) {
			return Err(RemoveSendTrackError::NoSendTrackWithId(id));
		}
		Ok(())
	}

	pub fn add_group_id(&mut self, id: GroupId) -> Result<(), AddGroupError> {
		if self.active_group_ids.len() >= self.active_group_ids.capacity() {
			return Err(AddGroupError::GroupLimitReached);
		}
		self.active_group_ids.insert(id);
		Ok(())
	}

	pub fn remove_group_id(&mut self, id: GroupId) -> Result<(), RemoveGroupError> {
		if !self.active_group_ids.remove(&id) {
			return Err(RemoveGroupError::NoGroupWithId(id));
		}
		Ok(())
	}

	pub fn add_metronome_id(&mut self, id: MetronomeId) -> Result<(), AddMetronomeError> {
		if self.active_metronome_ids.len() >= self.active_metronome_ids.capacity() {
			return Err(AddMetronomeError::MetronomeLimitReached);
		}
		self.active_metronome_ids.insert(id);
		Ok(())
	}

	pub fn remove_metronome_id(&mut self, id: MetronomeId) -> Result<(), RemoveMetronomeError> {
		if !self.active_metronome_ids.remove(&id) {
			return Err(RemoveMetronomeError::NoMetronomeWithId(id));
		}
		Ok(())
	}

	pub fn add_stream_id(&mut self, id: AudioStreamId) -> Result<(), AddStreamError> {
		if self.active_stream_ids.len() >= self.active_stream_ids.capacity() {
			return Err(AddStreamError::StreamLimitReached);
		}
		self.active_stream_ids.insert(id);
		Ok(())
	}

	pub fn remove_stream_id(&mut self, id: AudioStreamId) -> Result<(), RemoveStreamError> {
		if !self.active_stream_ids.remove(&id) {
			return Err(RemoveStreamError::NoStreamWithId(id));
		}
		Ok(())
	}
}
