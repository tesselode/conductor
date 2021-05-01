use std::sync::{Arc, Mutex};

use ringbuf::Producer;
use thiserror::Error;

use super::Command;

/// Something that can go wrong when sending a command to the
/// audio thread.
#[derive(Debug, Error)]
pub enum CommandError {
	/// The command queue is full.
	#[error("Commands cannot be sent to the audio thread because the command queue is full")]
	CommandQueueFull,
	/// A thread panicked while using the command producer.
	#[error("The command producer cannot be used because a thread panicked while borrowing it.")]
	MutexPoisoned,
}

#[derive(Clone)]
pub(crate) struct CommandProducer {
	producer: Arc<Mutex<Producer<Command>>>,
}

impl CommandProducer {
	pub fn new(producer: Producer<Command>) -> Self {
		Self {
			producer: Arc::new(Mutex::new(producer)),
		}
	}

	pub fn push(&mut self, command: Command) -> Result<(), CommandError> {
		self.producer
			.lock()
			.map_err(|_| CommandError::MutexPoisoned)?
			.push(command)
			.map_err(|_| CommandError::CommandQueueFull)
	}
}
