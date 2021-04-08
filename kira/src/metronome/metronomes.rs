use basedrop::Owned;

use crate::{
	command::MetronomeCommand,
	metronome::{Metronome, MetronomeId},
	parameter::Parameters,
	vec_map::VecMap,
};

pub(crate) struct Metronomes {
	metronomes: VecMap<MetronomeId, Owned<Metronome>>,
}

impl Metronomes {
	pub fn new(capacity: usize) -> Self {
		Self {
			metronomes: VecMap::new(capacity),
		}
	}

	pub fn get(&self, id: MetronomeId) -> Option<&Owned<Metronome>> {
		self.metronomes.get(&id)
	}

	pub fn run_command(&mut self, command: MetronomeCommand) {
		match command {
			MetronomeCommand::AddMetronome(id, metronome) => {
				self.metronomes.insert(id, metronome).ok();
			}
			MetronomeCommand::RemoveMetronome(id) => {
				self.metronomes.remove(&id);
			}
			MetronomeCommand::SetMetronomeTempo(id, tempo) => {
				if let Some(metronome) = self.metronomes.get_mut(&id) {
					metronome.set_tempo(tempo);
				}
			}
			MetronomeCommand::StartMetronome(id) => {
				if let Some(metronome) = self.metronomes.get_mut(&id) {
					metronome.start();
				}
			}
			MetronomeCommand::PauseMetronome(id) => {
				if let Some(metronome) = self.metronomes.get_mut(&id) {
					metronome.pause();
				}
			}
			MetronomeCommand::StopMetronome(id) => {
				if let Some(metronome) = self.metronomes.get_mut(&id) {
					metronome.stop();
				}
			}
		}
	}

	pub fn update(&mut self, dt: f64, parameters: &Parameters) {
		for metronome in &mut self.metronomes {
			metronome.update(dt, parameters);
		}
	}
}
