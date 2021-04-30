pub mod data;

use std::sync::{atomic::AtomicUsize, Arc};

use atomig::Ordering;

use self::data::SoundData;

static NEXT_SOUND_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundId(usize);

impl SoundId {
	pub(crate) fn new() -> Self {
		Self(NEXT_SOUND_INDEX.fetch_add(1, Ordering::SeqCst))
	}
}

pub struct Sound {
	data: Arc<dyn SoundData>,
}

impl Sound {
	pub fn new(data: Arc<dyn SoundData>) -> Self {
		Self { data }
	}
}
