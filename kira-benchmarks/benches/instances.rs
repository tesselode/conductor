use std::{f32::consts::PI, vec};

use criterion::{criterion_group, criterion_main, Criterion};
use kira::{
	frame::Frame,
	manager::{AudioManager, AudioManagerSettings},
	sound::{
		data::static_sound::StaticSoundData, instance::settings::InstanceSettings, SoundSettings,
	},
};

fn create_test_sound(num_samples: usize) -> StaticSoundData {
	const SAMPLE_RATE: u32 = 48000;
	let mut sine_samples = vec![];
	let mut phase = 0.0;
	for _ in 0..num_samples {
		sine_samples.push(Frame::from_mono((phase * 2.0 * PI).sin()));
		phase += 440.0 / SAMPLE_RATE as f32;
	}
	StaticSoundData::from_frames(SAMPLE_RATE, sine_samples)
}

fn instances_benchmark(c: &mut Criterion) {
	let mut benchmark_group = c.benchmark_group("instances");

	benchmark_group.bench_function("simple", |b| {
		const NUM_INSTANCES: usize = 100_000;
		let (mut audio_manager, mut backend) =
			AudioManager::new_without_audio_thread(AudioManagerSettings {
				num_commands: NUM_INSTANCES,
				..Default::default()
			});
		// add a test sound
		let mut sound_handle = audio_manager
			.add_sound(
				create_test_sound(48000),
				SoundSettings::new()
					.num_instances(NUM_INSTANCES)
					.default_loop_start(0.0),
			)
			.unwrap();
		backend.process();
		// start a bunch of instances
		for _ in 0..NUM_INSTANCES {
			sound_handle.play(Default::default()).unwrap();
		}
		backend.process();
		b.iter(|| backend.process());
		drop(backend);
		drop(audio_manager);
	});

	benchmark_group.bench_function("with parameters", |b| {
		const NUM_INSTANCES: usize = 100_000;
		let (mut audio_manager, mut backend) =
			AudioManager::new_without_audio_thread(AudioManagerSettings {
				num_commands: NUM_INSTANCES,
				..Default::default()
			});
		let parameter_1 = audio_manager.add_parameter(0.5).unwrap();
		let parameter_2 = audio_manager.add_parameter(0.5).unwrap();
		let parameter_3 = audio_manager.add_parameter(0.5).unwrap();
		// add a test sound
		let mut sound_handle = audio_manager
			.add_sound(
				create_test_sound(48000),
				SoundSettings::new()
					.num_instances(NUM_INSTANCES)
					.default_loop_start(0.0),
			)
			.unwrap();
		backend.process();
		// start a bunch of instances
		for _ in 0..NUM_INSTANCES {
			sound_handle
				.play(
					InstanceSettings::new()
						.volume(&parameter_1)
						.playback_rate(&parameter_2)
						.panning(&parameter_3),
				)
				.unwrap();
		}
		backend.process();
		b.iter(|| backend.process());
		drop(backend);
		drop(audio_manager);
	});
}

criterion_group!(benches, instances_benchmark);
criterion_main!(benches);
