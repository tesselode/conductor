use crate::{
	audio_stream::{AudioStream, AudioStreamId},
	command::StreamCommand,
	manager::TrackIndex,
	vec_map::VecMap,
};

use super::mixer::Mixer;

use basedrop::Owned;

pub(crate) struct Streams {
	streams: VecMap<AudioStreamId, (TrackIndex, Owned<Box<dyn AudioStream>>)>,
}

impl Streams {
	pub fn new(capacity: usize) -> Self {
		Self {
			streams: VecMap::new(capacity),
		}
	}

	pub fn run_command(&mut self, command: StreamCommand) {
		match command {
			StreamCommand::AddStream(stream_id, track_id, stream) => {
				self.streams.insert(stream_id, (track_id, stream)).ok();
			}
			StreamCommand::RemoveStream(stream_id) => {
				self.streams.remove(&stream_id);
			}
		}
	}

	pub fn process(&mut self, dt: f64, mixer: &mut Mixer) {
		for (track, stream) in &mut self.streams {
			mixer.add_input(*track, stream.next(dt));
		}
	}
}
