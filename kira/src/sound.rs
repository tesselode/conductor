pub mod data;
pub mod handle;
pub mod instance;

use std::sync::Arc;

use self::data::SoundData;

pub(crate) struct Sound {
	data: Arc<dyn SoundData>,
}

impl Sound {
	pub fn new(data: Arc<dyn SoundData>) -> Self {
		Self { data }
	}
}
