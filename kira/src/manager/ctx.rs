use std::sync::atomic::AtomicU32;

pub struct AudioContext {
	pub sample_rate: AtomicU32,
}

impl AudioContext {
	pub fn new(sample_rate: u32) -> Self {
		Self {
			sample_rate: AtomicU32::new(sample_rate),
		}
	}
}
