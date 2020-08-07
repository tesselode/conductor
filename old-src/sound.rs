use crate::{project::SoundSettings, stereo_sample::StereoSample, time::Time};
use lewton::{inside_ogg::OggStreamReader, samples::Samples};
use std::{error::Error, fs::File, path::Path};

pub struct Sound {
	sample_rate: u32,
	samples: Vec<StereoSample>,
	duration: f32,
	pub tempo: Option<f32>,
	pub default_loop_start: Time,
	pub default_loop_end: Time,
}

impl Sound {
	pub fn new(sample_rate: u32, samples: Vec<StereoSample>, settings: SoundSettings) -> Self {
		let duration = samples.len() as f32 / sample_rate as f32;
		Self {
			sample_rate,
			samples,
			duration,
			tempo: settings.tempo,
			default_loop_start: settings.default_loop_start.unwrap_or(Time::Seconds(0.0)),
			default_loop_end: settings.default_loop_end.unwrap_or(Time::Seconds(duration)),
		}
	}

	pub fn from_ogg_file(path: &Path, settings: SoundSettings) -> Result<Self, Box<dyn Error>> {
		let mut reader = OggStreamReader::new(File::open(path)?)?;
		let mut samples = vec![];
		while let Some(packet) = reader.read_dec_packet_generic::<Vec<Vec<f32>>>()? {
			let num_channels = packet.len();
			let num_samples = packet.num_samples();
			match num_channels {
				1 => {
					for i in 0..num_samples {
						samples.push(StereoSample::from_mono(packet[0][i]));
					}
				}
				2 => {
					for i in 0..num_samples {
						samples.push(StereoSample::new(packet[0][i], packet[1][i]));
					}
				}
				_ => {
					panic!("Only mono and stereo audio can be loaded");
				}
			}
		}
		Ok(Self::new(
			reader.ident_hdr.audio_sample_rate,
			samples,
			settings,
		))
	}

	pub fn duration(&self) -> f32 {
		self.duration
	}

	pub fn get_sample_at_position(&self, position: f32) -> StereoSample {
		let sample_position = self.sample_rate as f32 * position;
		let x = sample_position % 1.0;
		let current_sample_index = sample_position as usize;
		let y0 = if current_sample_index == 0 {
			StereoSample::from_mono(0.0)
		} else {
			*self
				.samples
				.get(current_sample_index - 1)
				.unwrap_or(&StereoSample::from_mono(0.0))
		};
		let y1 = *self
			.samples
			.get(current_sample_index)
			.unwrap_or(&StereoSample::from_mono(0.0));
		let y2 = *self
			.samples
			.get(current_sample_index + 1)
			.unwrap_or(&StereoSample::from_mono(0.0));
		let y3 = *self
			.samples
			.get(current_sample_index + 2)
			.unwrap_or(&StereoSample::from_mono(0.0));
		let c0 = y1;
		let c1 = (y2 - y0) * 0.5;
		let c2 = y0 - y1 * 2.5 + y2 * 2.0 - y3 * 0.5;
		let c3 = (y3 - y0) * 0.5 + (y1 - y2) * 1.5;
		((c3 * x + c2) * x + c1) * x + c0
	}
}