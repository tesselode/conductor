use crate::command::{
	producer::{CommandError, CommandProducer},
	Command, ParameterCommand,
};

use super::{tween::Tween, ParameterId};

pub struct ParameterHandle {
	id: ParameterId,
	command_producer: CommandProducer,
}

impl ParameterHandle {
	pub(crate) fn new(id: ParameterId, command_producer: CommandProducer) -> Self {
		Self {
			id,
			command_producer,
		}
	}

	pub fn id(&self) -> ParameterId {
		self.id
	}

	pub fn set(&mut self, value: f64, tween: impl Into<Option<Tween>>) -> Result<(), CommandError> {
		self.command_producer
			.push(Command::Parameter(ParameterCommand::SetParameter {
				id: self.id,
				value,
				tween: tween.into(),
			}))
	}
}
