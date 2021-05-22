mod backend;
mod error;

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};

use crate::manager::{backend::Backend, error::SetupError};

pub struct AudioManager {
	_stream: Stream,
}

impl AudioManager {
	pub fn new() -> Result<Self, SetupError> {
		let host = cpal::default_host();
		let device = host
			.default_output_device()
			.ok_or(SetupError::NoDefaultOutputDevice)?;
		let config = device.default_output_config()?.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let mut backend = Backend::new(sample_rate);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				for frame in data.chunks_exact_mut(channels as usize) {
					#[cfg(feature = "assert_no_alloc")]
					let out = assert_no_alloc::assert_no_alloc(|| backend.process());
					#[cfg(not(feature = "assert_no_alloc"))]
					let out = backend.process();
					if channels == 1 {
						frame[0] = (out.left + out.right) / 2.0;
					} else {
						frame[0] = out.left;
						frame[1] = out.right;
					}
				}
			},
			move |_| {},
		)?;
		stream.play()?;
		Ok(Self { _stream: stream })
	}
}
