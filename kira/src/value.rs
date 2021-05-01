use std::ops::Range;

use rand::{thread_rng, Rng};

use crate::parameter::{
	handle::ParameterHandle, mapping::Mapping, parameters::Parameters, ParameterId,
};

/// A value that something can be set to.
#[derive(Debug, Copy, Clone)]
pub enum Value<T: From<f64> + Into<f64> + Copy> {
	/// A fixed value.
	Fixed(T),
	/// The current value of a parameter.
	Parameter(ParameterId, Mapping),
	/// A random value within a range.
	Random(T, T),
}

impl<T: From<f64> + Into<f64> + Copy> From<T> for Value<T> {
	fn from(value: T) -> Self {
		Self::Fixed(value)
	}
}

impl<T: From<f64> + Into<f64> + Copy> From<ParameterId> for Value<T> {
	fn from(id: ParameterId) -> Self {
		Self::Parameter(id, Mapping::default())
	}
}

impl<T: From<f64> + Into<f64> + Copy> From<&ParameterHandle> for Value<T> {
	fn from(handle: &ParameterHandle) -> Self {
		Self::Parameter(handle.id(), Mapping::default())
	}
}

impl<T: From<f64> + Into<f64> + Copy> From<Range<T>> for Value<T> {
	fn from(range: Range<T>) -> Self {
		Self::Random(range.start, range.end)
	}
}

/// A wrapper around [`Value`](crate::Value)s that remembers the last valid raw value.
///
/// You'll only need to use this if you're writing your own effects.
#[derive(Debug, Copy, Clone)]
pub struct CachedValue<T: From<f64> + Into<f64> + Copy> {
	value: Value<T>,
	last_value: T,
	min: Option<T>,
	max: Option<T>,
}

impl<T: From<f64> + Into<f64> + Copy> CachedValue<T> {
	/// Creates a `CachedValue` with an initial value setting
	/// and a default raw value to fall back on.
	pub fn new(value: Value<T>, default_value: T) -> Self {
		Self {
			value,
			last_value: match value {
				Value::Fixed(value) => value,
				Value::Parameter(_, _) => default_value,
				Value::Random(lower, upper) => Self::pick_random(lower, upper),
			},
			min: None,
			max: None,
		}
	}

	/// Sets the minimum valid value of this `CachedValue`.
	pub fn with_min(self, min: T) -> Self {
		Self {
			min: Some(min),
			..self
		}
	}

	/// Sets the maximum valid value of this `CachedValue`.
	pub fn with_max(self, max: T) -> Self {
		Self {
			max: Some(max),
			..self
		}
	}

	/// Sets the min and max valid values of this `CachedValue`.
	pub fn with_valid_range(self, range: Range<T>) -> Self {
		Self {
			min: Some(range.start),
			max: Some(range.end),
			..self
		}
	}

	fn pick_random(lower: T, upper: T) -> T {
		let lower: f64 = lower.into();
		let upper: f64 = upper.into();
		thread_rng().gen_range(lower..upper).into()
	}

	/// Sets the value.
	pub fn set(&mut self, value: Value<T>) {
		self.value = value;
		match value {
			Value::Fixed(value) => {
				self.last_value = value;
			}
			Value::Random(lower, upper) => {
				self.last_value = Self::pick_random(lower, upper);
			}
			_ => {}
		}
	}

	/// If the value is set to a parameter, updates the raw value
	/// from the parameter (if it exists).
	pub fn update(&mut self, parameters: &Parameters) {
		match self.value {
			Value::Parameter(id, mapping) => {
				if let Some(parameter) = parameters.get(id) {
					self.last_value = mapping.map(parameter.value()).into();
				}
			}
			_ => {}
		}
	}

	/// Gets the last valid raw value.
	pub fn value(&self) -> T {
		let mut value: f64 = self.last_value.into();
		if let Some(min) = self.min {
			value = value.max(min.into());
		}
		if let Some(max) = self.max {
			value = value.min(max.into());
		}
		value.into()
	}
}
