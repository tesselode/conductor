use std::f32::consts::TAU;

use crate::frame::Frame;

pub struct Backend {
	sample_rate: u32,
	dt: f64,
	phase: f64,
}

impl Backend {
	pub fn new(sample_rate: u32) -> Self {
		Self {
			sample_rate,
			dt: 1.0 / sample_rate as f64,
			phase: 0.0,
		}
	}

	pub fn process(&mut self) -> Frame {
		self.phase += 440.0 * self.dt;
		Frame::from_mono(0.25 * (self.phase as f32 * TAU).sin())
	}
}
