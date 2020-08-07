use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_SOUND_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SoundId {
	index: usize,
}

impl SoundId {
	pub fn new() -> Self {
		let index = NEXT_SOUND_INDEX.fetch_add(1, Ordering::Relaxed);
		Self { index }
	}
}

static NEXT_INSTANCE_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct InstanceId {
	index: usize,
}

impl InstanceId {
	pub fn new() -> Self {
		let index = NEXT_INSTANCE_INDEX.fetch_add(1, Ordering::Relaxed);
		Self { index }
	}
}