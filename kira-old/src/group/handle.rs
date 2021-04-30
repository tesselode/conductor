//! An interface for controlling groups.

use crate::{
	command::{
		producer::{CommandError, CommandProducer},
		InstanceCommand, SequenceCommand,
	},
	instance::{PauseInstanceSettings, ResumeInstanceSettings, StopInstanceSettings},
};

use super::GroupId;

#[derive(Debug, Clone)]
/// Allows you to control a group.
pub struct GroupHandle {
	id: GroupId,
	command_producer: CommandProducer,
}

impl GroupHandle {
	pub(crate) fn new(id: GroupId, command_producer: CommandProducer) -> Self {
		Self {
			id,
			command_producer,
		}
	}

	/// Returns the ID of the group.
	pub fn id(&self) -> GroupId {
		self.id
	}

	/// Pauses all instances of sounds, arrangements, and sequences in this group.
	pub fn pause(&mut self, settings: PauseInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::PauseGroup(self.id().into(), settings).into())?;
		self.command_producer
			.push(SequenceCommand::PauseGroup(self.id().into()).into())?;
		Ok(())
	}

	/// Resumes all instances of sounds, arrangements, and sequences in this group.
	pub fn resume(&mut self, settings: ResumeInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::ResumeGroup(self.id().into(), settings).into())?;
		self.command_producer
			.push(SequenceCommand::ResumeGroup(self.id().into()).into())?;
		Ok(())
	}

	/// Stops all instances of sounds, arrangements, and sequences in this group.
	pub fn stop(&mut self, settings: StopInstanceSettings) -> Result<(), CommandError> {
		self.command_producer
			.push(InstanceCommand::StopGroup(self.id().into(), settings).into())?;
		self.command_producer
			.push(SequenceCommand::StopGroup(self.id().into()).into())?;
		Ok(())
	}
}
