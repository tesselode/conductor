use std::{f32::consts::PI, vec};

use criterion::{criterion_group, criterion_main, Criterion};
use kira::{
	frame::Frame,
	manager::{AudioManager, AudioManagerSettings},
	sound::{data::static_sound::StaticSoundData, SoundSettings},
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

	benchmark_group.bench_function("one sound", |b| {
		const NUM_INSTANCES: usize = 100_000;
		let (mut audio_manager, mut backend) =
			AudioManager::new_without_audio_thread(AudioManagerSettings {
				num_commands: NUM_INSTANCES,
				..Default::default()
			});
		// add a test sound
		let mut sound_handle = audio_manager
			.add_sound(
				create_test_sound(4800000),
				SoundSettings {
					num_instances: NUM_INSTANCES,
				},
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
}

criterion_group!(benches, instances_benchmark);
criterion_main!(benches);
