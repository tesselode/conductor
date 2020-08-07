mod backend;

use crate::{
	error::ConductorError,
	project::{Project, SoundId},
	sequence::Sequence,
	time::Time,
};
use backend::{Backend, Command};
use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Stream,
};
use ringbuf::{Consumer, Producer, RingBuffer};
use std::error::Error;

const COMMAND_QUEUE_CAPACITY: usize = 100;
const EVENT_QUEUE_CAPACITY: usize = 100;

#[derive(Debug)]
pub enum Event {
	MetronomeInterval(f32),
}

#[derive(Debug)]
pub struct InstanceSettings {
	pub position: Time,
	pub volume: f32,
	pub pitch: f32,
}

impl Default for InstanceSettings {
	fn default() -> Self {
		Self {
			position: Time::Seconds(0.0),
			volume: 1.0,
			pitch: 1.0,
		}
	}
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct InstanceHandle {
	index: usize,
}

#[derive(Default, Debug)]
pub struct LooperSettings {
	start: Option<Time>,
	end: Option<Time>,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct LooperHandle {
	index: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct SequenceHandle {
	index: usize,
}

pub struct AudioManagerSettings {
	pub num_instances: usize,
	pub num_loopers: usize,
	pub num_sequences: usize,
	pub tempo: f32,
	pub metronome_event_intervals: Vec<f32>,
}

impl Default for AudioManagerSettings {
	fn default() -> Self {
		Self {
			num_instances: 100,
			num_loopers: 50,
			num_sequences: 50,
			tempo: 120.0,
			metronome_event_intervals: vec![1.0],
		}
	}
}

pub struct AudioManager {
	command_producer: Producer<Command>,
	event_consumer: Consumer<Event>,
	_stream: Stream,
	next_instance_handle_index: usize,
	next_looper_handle_index: usize,
	next_sequence_handle_index: usize,
}

impl AudioManager {
	pub fn new(project: Project, settings: AudioManagerSettings) -> Result<Self, Box<dyn Error>> {
		let host = cpal::default_host();
		let device = host.default_output_device().unwrap();
		let mut supported_configs_range = device.supported_output_configs().unwrap();
		let supported_config = supported_configs_range
			.next()
			.unwrap()
			.with_max_sample_rate();
		let config = supported_config.config();
		let sample_rate = config.sample_rate.0;
		let channels = config.channels;
		let (command_producer, command_consumer) = RingBuffer::new(COMMAND_QUEUE_CAPACITY).split();
		let (event_producer, event_consumer) = RingBuffer::new(EVENT_QUEUE_CAPACITY).split();
		let mut backend = Backend::new(
			sample_rate,
			project,
			settings,
			command_consumer,
			event_producer,
		);
		let stream = device.build_output_stream(
			&config,
			move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
				for frame in data.chunks_exact_mut(channels as usize) {
					let out = backend.process();
					frame[0] = out.left;
					frame[1] = out.right;
				}
			},
			move |_| {},
		)?;
		stream.play()?;
		Ok(Self {
			command_producer,
			event_consumer,
			_stream: stream,
			next_instance_handle_index: 0,
			next_looper_handle_index: 0,
			next_sequence_handle_index: 0,
		})
	}

	pub fn play_sound(
		&mut self,
		sound_id: SoundId,
		settings: InstanceSettings,
	) -> Result<InstanceHandle, ConductorError> {
		let instance_handle = InstanceHandle {
			index: self.next_instance_handle_index,
		};
		self.next_instance_handle_index += 1;
		match self
			.command_producer
			.push(Command::PlaySound(sound_id, instance_handle, settings))
		{
			Ok(_) => Ok(instance_handle),
			Err(_) => Err(ConductorError::SendCommand),
		}
	}

	pub fn set_instance_volume(
		&mut self,
		instance_handle: InstanceHandle,
		volume: f32,
	) -> Result<(), ConductorError> {
		match self
			.command_producer
			.push(Command::SetInstanceVolume(instance_handle, volume))
		{
			Ok(_) => Ok(()),
			Err(_) => Err(ConductorError::SendCommand),
		}
	}

	pub fn set_instance_pitch(
		&mut self,
		instance_handle: InstanceHandle,
		pitch: f32,
	) -> Result<(), ConductorError> {
		match self
			.command_producer
			.push(Command::SetInstancePitch(instance_handle, pitch))
		{
			Ok(_) => Ok(()),
			Err(_) => Err(ConductorError::SendCommand),
		}
	}

	pub fn loop_sound(
		&mut self,
		sound_id: SoundId,
		settings: LooperSettings,
	) -> Result<LooperHandle, ConductorError> {
		let handle = LooperHandle {
			index: self.next_looper_handle_index,
		};
		self.next_looper_handle_index += 1;
		match self
			.command_producer
			.push(Command::LoopSound(sound_id, handle, settings))
		{
			Ok(_) => Ok(handle),
			Err(_) => Err(ConductorError::SendCommand),
		}
	}

	pub fn start_metronome(&mut self) {
		match self.command_producer.push(Command::StartMetronome) {
			Ok(_) => {}
			Err(_) => {}
		}
	}

	pub fn start_sequence(&mut self, sequence: Sequence) -> Result<SequenceHandle, ConductorError> {
		let handle = SequenceHandle {
			index: self.next_sequence_handle_index,
		};
		self.next_sequence_handle_index += 1;
		match self
			.command_producer
			.push(Command::StartSequence(sequence, handle))
		{
			Ok(_) => Ok(handle),
			Err(_) => Err(ConductorError::SendCommand),
		}
	}

	pub fn events(&mut self) -> Vec<Event> {
		let mut events = vec![];
		while let Some(event) = self.event_consumer.pop() {
			events.push(event);
		}
		events
	}
}