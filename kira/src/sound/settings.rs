pub struct SoundSettings {
	pub num_instances: usize,
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
}

impl Default for SoundSettings {
	fn default() -> Self {
		Self { num_instances: 10 }
	}
}
