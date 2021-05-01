pub mod handle;
pub mod mapping;
pub mod parameters;
pub mod tween;

use std::sync::atomic::{AtomicUsize, Ordering};

use self::{handle::ParameterHandle, tween::Tween};

static NEXT_PARAMETER_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParameterId(usize);

impl ParameterId {
	pub(crate) fn new() -> Self {
		Self(NEXT_PARAMETER_INDEX.fetch_add(1, Ordering::SeqCst))
	}
}

impl From<&ParameterHandle> for ParameterId {
	fn from(handle: &ParameterHandle) -> Self {
		handle.id()
	}
}

#[derive(Debug, Copy, Clone)]
struct TweenState {
	tween: Tween,
	start: f64,
	end: f64,
	time: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Parameter {
	value: f64,
	tween_state: Option<TweenState>,
}

impl Parameter {
	pub(crate) fn new(value: f64) -> Self {
		Self {
			value,
			tween_state: None,
		}
	}

	pub(crate) fn value(&self) -> f64 {
		self.value
	}

	pub(crate) fn set(&mut self, target: f64, tween: Option<Tween>) {
		if let Some(tween) = tween {
			self.tween_state = Some(TweenState {
				tween,
				start: self.value,
				end: target,
				time: 0.0,
			});
		} else {
			self.value = target;
		}
	}

	pub(crate) fn update(&mut self, dt: f64) -> bool {
		if let Some(tween_state) = &mut self.tween_state {
			tween_state.time += dt;
			self.value =
				tween_state
					.tween
					.tween(tween_state.start, tween_state.end, tween_state.time);
			if tween_state.time >= tween_state.tween.duration {
				self.tween_state = None;
				return true;
			}
		}
		false
	}
}
