use std::sync::Arc;

use super::data::SoundData;

#[derive(Clone)]
pub struct SoundHandle {
	data: Arc<dyn SoundData>,
}

impl SoundHandle {
	pub fn new(data: Arc<dyn SoundData>) -> Self {
		Self { data }
	}

	pub(crate) fn data(&self) -> &Arc<dyn SoundData> {
		&self.data
	}
}
