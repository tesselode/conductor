use std::sync::Arc;

use atomig::Ordering;
use ringbuf::Consumer;

use crate::frame::Frame;

use super::{command::Command, ctx::AudioContext};

pub struct Backend {
	ctx: Arc<AudioContext>,
	dt: f64,
	command_consumer: Consumer<Command>,
}

impl Backend {
	pub fn new(ctx: Arc<AudioContext>, command_consumer: Consumer<Command>) -> Self {
		let dt = 1.0 / ctx.sample_rate.load(Ordering::SeqCst) as f64;
		Self { ctx, dt, command_consumer }
	}

	pub fn process(&mut self) -> Frame {
		Frame::from_mono(0.0)
	}
}
