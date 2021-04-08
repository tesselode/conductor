use basedrop::Owned;

use crate::{command::GroupCommand, vec_map::VecMap};

use super::{Group, GroupId};

pub(crate) struct Groups {
	groups: VecMap<GroupId, Owned<Group>>,
}

impl Groups {
	pub fn new(capacity: usize) -> Self {
		Self {
			groups: VecMap::new(capacity),
		}
	}

	pub fn get(&self, id: GroupId) -> Option<&Owned<Group>> {
		self.groups.get(&id)
	}

	pub fn run_command(&mut self, command: GroupCommand) {
		match command {
			GroupCommand::AddGroup(id, group) => {
				self.groups.insert(id, group).ok();
			}
			GroupCommand::RemoveGroup(id) => {
				self.groups.remove(&id);
			}
		}
	}
}
